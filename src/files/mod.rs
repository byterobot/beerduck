use std::collections::HashSet;
use anyhow::Error;
use once_cell::sync::Lazy;

pub mod asciidoc;
pub mod posts;
pub mod category;
pub mod page;

pub async fn render_web() -> Result<(), Error> {
    // 遍历文件,

    // 按目录结构"复制"文件到临时目录? || 按目录结构加载文件?

    // 临时目录中生成各分类的索引 _index.adoc 和 总分类索引 category.adoc, 如果已存在 _index.adoc 则不生成
    // category 根目录下生成 category.adoc; abc 目录在 category 目录下生成 abc.adoc

    // 渲染 html, 提取有用内容, 用对应模板组装成最终的 html
    // 将 html 文件移动到最终的位置
    // 删除临时目录


    // 解决图片路径问题
    Ok(())
}

// static EXTS: Lazy<HashSet<String>> = Lazy::new(|| [""])
//
// pub fn valid_ext(file_name: &str) -> bool {
//
// }




