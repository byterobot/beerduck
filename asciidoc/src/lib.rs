pub use dom::*;
pub use convert_html5::convert;

// use crate::convert::html5;
// use crate::hybrid::Hybrid;

pub mod hybrid;
mod dom;
mod convert_html5;

// pub mod convert;

// pub fn convert(text: &str) -> Result<String, Error> {
//     Ok(html5::convert(&convert::convert(Hybrid::parse(text).text())?))
// }
