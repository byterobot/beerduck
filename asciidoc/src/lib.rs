use anyhow::Error;

pub use dom::*;

use crate::convert::html5;
use crate::hybrid::Hybrid;

pub mod hybrid;
mod dom;
pub mod convert;

pub fn convert(text: &str) -> Result<String, Error> {
    Ok(html5::convert(&convert::convert(Hybrid::parse(text).text())?))
}