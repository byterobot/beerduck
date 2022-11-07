#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::Error;
use async_std::prelude::StreamExt;
use async_std::task::block_on;
use log::{debug, error, info, LevelFilter};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::{Lazy, OnceCell};
use tide::{Endpoint, Request, sse, StatusCode};
use tide_websockets::{Message, WebSocket, WebSocketConnection};

use config::{parent, workspace};

use crate::middleware::HtmlMiddleware;

mod middleware;

pub async fn start_server() -> Result<(), Error> {
    let listener = listen_modified()?;
    let mut app = tide::new();
    app.at("/ws/:uuid").get(WebSocket::new(ws_handler));
    app.at("/assets/images").serve_dir(parent().join(&workspace().assets.images))?;
    app.at("/static").serve_dir(parent().join(&workspace().theme.static_.self_dir))?;
    app.at("*").with(HtmlMiddleware::new()).get(|req: Request<_>| async move {
        match data::endpoint(req.url().path()) {
            None => Err(tide::Error::from_str(StatusCode::NotFound, "404 Not Found")),
            Some(html) => Ok(html)
        }
    });
    info!("listening at 0.0.0.0:2020");
    app.listen("0.0.0.0:2020").await?;
    Ok(())
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
    watcher.watch(&parent().join(&workspace().theme.self_dir), RecursiveMode::Recursive)?;
    watcher.watch(&parent().join(&workspace().assets.images), RecursiveMode::Recursive)?;
    Ok(watcher)
}

static STREAMS: Lazy<Mutex<HashMap<String, WebSocketConnection>>> = Lazy::new(|| Mutex::new(HashMap::new()));

async fn on_changed(event: Event) -> Result<(), Error> {
    println!("file changed: {:?}", event);
    for v in STREAMS.lock().expect("lock STREAMS failed").values() {
        let file = event.paths.first().map(|v| v.to_str().unwrap()).unwrap_or("");
        if let Err(e) = v.send_string(file.into()).await {
            error!("send to client failed: {:?}", e);
        }
    }
    Ok(())
}

async fn ws_handler<S>(request: Request<S>, mut stream: WebSocketConnection) -> tide::Result<()> {
    // open a connection
    let uuid = request.param("uuid")?;
    STREAMS.lock().expect("lock STREAMS failed").insert(uuid.to_string(), stream.clone());

    while let Some(v) = stream.next().await {
        match v {
            Ok(message) => debug!("{:?}", message),
            Err(e) => error!("{:?}", e),
        }
    }
    // while let Some(Ok(Message::Text(input))) = stream.next().await {
    //     let output: String = input.chars().rev().collect();
    //     stream.send_string(format!("{} | {}", &input, &output)).await?;
    // }

    // connection closed
    STREAMS.lock().expect("lock STREAMS failed").remove(uuid);
    Ok(())
}
