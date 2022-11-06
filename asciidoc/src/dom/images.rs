use std::str;

use log::error;
use tl::{NodeHandle, VDom};

use config::{live_mode, make_relative_path, workspace};

pub fn get_content_images(dom: &VDom) -> Option<Vec<String>> {
    let vec = content_image_nodes(dom)?.into_iter()
        .filter_map(|n| get_image(dom, n))
        .collect::<Vec<String>>();
    Some(vec)
}

fn content_image_nodes(dom: &VDom) -> Option<Vec<NodeHandle>> {
    let nodes = dom.get_element_by_id("content")?
        .get(dom.parser())?
        .as_tag()?
        .query_selector(dom.parser(), "img[src]")?
        .collect::<Vec<NodeHandle>>();
    Some(nodes)
}

fn get_image(dom: &VDom, n: NodeHandle) -> Option<String> {
    let bytes = n.get(dom.parser())?.as_tag()?.attributes().get("src")??;
    str::from_utf8(bytes.as_bytes())
        .map_err(|e| error!("parse img src error: {}", e))
        .ok()
        .map(|v| v.to_string())
}

pub fn resolve_images(dom: &mut VDom) -> Option<()> {
    for n in content_image_nodes(dom)? {
        modify_image_path(dom, n);
    }
    None
}

fn modify_image_path(dom: &mut VDom, n: NodeHandle) -> Option<()> {
    let bytes = n.get_mut(dom.parser_mut())?.as_tag_mut()?.attributes_mut()
        .get_mut("src")??;
    let src = str::from_utf8(bytes.as_bytes())
        .map_err(|e| error!("parse img src error: {}", e)).ok()?;
    let _ = bytes.set(&*resolve_content_image(src));
    None
}

fn resolve_content_image(path: &str) -> String {
    match live_mode() {
        true => format!("/{}/{}", workspace().assets.images, make_relative_path(path)),
        _ => {
            let dir = workspace().publish.static_.images
                .replacen(&workspace().publish.self_dir, "", 1);
            format!("/{}/{}", dir, make_relative_path(path))
        },
    }
}
