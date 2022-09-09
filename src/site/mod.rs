use anyhow::Error;
use crate::config::CONFIG;
use crate::dict::DICT;

pub(crate) mod article;

pub fn generate_site() -> Result<(), Error> {
    for (k, v) in DICT.adoc_map() {
        let a = article::render(&v.adoc_absolute, &v.html_absolute)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::site::generate_site;

    #[test]
    fn test() {
        generate_site().unwrap();
    }
}