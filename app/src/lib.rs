#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

use std::collections::HashMap;
use std::time::{Duration, Instant};

use anyhow::Error;
use async_std::prelude::StreamExt;
use async_std::sync::Mutex;
use async_std::task::block_on;
use log::{debug, error, info};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use percent_encoding::percent_decode_str;
use tide::{Request, StatusCode};
use tide_websockets::{WebSocket, WebSocketConnection};

use config::{parent, workspace};

use crate::middleware::HtmlMiddleware;

mod middleware;

static STREAMS: Lazy<Mutex<HashMap<String, WebSocketConnection>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static EVENT_INSTANT: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));

pub async fn start_server() -> Result<(), Error> {
    EVENT_INSTANT.lock().await;
    let listener = listen_modified()?;
    let mut app = tide::new();
    app.at("/ws/:uuid").get(WebSocket::new(ws_handler));
    app.at("/assets/images").serve_dir(parent().join(&workspace().assets.images))?;
    app.at("/static").serve_dir(parent().join(&workspace().theme.static_.self_dir))?;
    app.at("").with(HtmlMiddleware::new()).get(endpoint);
    app.at("*").with(HtmlMiddleware::new()).get(endpoint);
    info!("listening at 0.0.0.0:2020");
    app.listen("0.0.0.0:2020").await?;
    Ok(())
}

async fn endpoint(req: Request<()>) -> tide::Result<String> {
    let path = match percent_decode_str(req.url().path()).decode_utf8() {
        Ok(v) => v.to_string(),
        Err(e) => Err(tide::Error::from_str(StatusCode::NotFound, e))?,
    };

    match data::endpoint(path.as_str()) {
        None => Err(tide::Error::from_str(StatusCode::NotFound, "404 Not Found")),
        Some(html) => Ok(html)
    }
}

pub fn listen_modified() -> Result<RecommendedWatcher, Error> {
    let mut watcher = RecommendedWatcher::new(|e: Result<Event, notify::Error>| {
        if let Ok(event) = e {
            block_on(async {
                if let Err(e) = on_changed(event).await {
                    error!("process file modify event error: {}", e);
                }
            });
        }
    }, notify::Config::default())?;
    watcher.watch(&parent().join(&workspace().posts), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().theme.static_.js), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().theme.static_.css), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().theme.static_.images), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().theme.static_.fonts), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().assets.images), RecursiveMode::Recursive)?;
    Ok(watcher)
}

async fn ws_handler<S>(request: Request<S>, mut stream: WebSocketConnection) -> tide::Result<()> {
    // open a connection
    let uuid = request.param("uuid")?;
    STREAMS.lock().await.insert(uuid.to_string(), stream.clone());
    while let Some(v) = stream.next().await {
        match v {
            Ok(message) => debug!("{:?}", message),
            Err(e) => error!("{:?}", e),
        }
    }
    // connection closed
    STREAMS.lock().await.remove(uuid);
    Ok(())
}

// Control the number of concurrency
async fn on_changed(event: Event) -> Result<(), Error> {
    let mut instant = EVENT_INSTANT.lock().await;
    let now = Instant::now();
    if now.duration_since(*instant) < Duration::from_secs(1) {
        return Ok(());
    }
    *instant = now;
    send(event).await;
    Ok(())
}

async fn send(event: Event) {
    for v in STREAMS.lock().await.values() {
        let file = event.paths.first().map(|v| v.to_str().unwrap()).unwrap_or("");
        if let Err(e) = v.send_string(file.into()).await {
            error!("send to client failed: {:?}", e);
        }
    }
}
