#![allow(unused_imports, deprecated, unused_must_use, unused_mut, unused_variables, dead_code)]

pub mod config;
pub mod asciidoc;
pub mod page;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}