use async_trait::async_trait;
use tide::{Middleware, Next, Request};
use tide::http::mime;

#[derive(Debug, Clone, Default)]
pub(crate) struct HtmlMiddleware;

impl HtmlMiddleware {
    pub(crate) fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for HtmlMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        let mut res = next.run(req).await;
        let mut body = res.take_body().into_string().await?;
        body.push('\n');
        body.push_str(include_str!("js/auto_reload.html"));
        res.set_body(body);
        res.set_content_type(mime::HTML);
        Ok(res)
    }
}