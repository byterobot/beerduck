use anyhow::Error;

pub mod asciidoc;
pub mod posts;
pub mod category;
pub mod page;
pub mod render;
pub mod template;

pub async fn render_web() -> Result<(), Error> {
    // 解决图片路径问题
    Ok(())
}

