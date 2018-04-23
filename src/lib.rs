#![deny(missing_docs)]

//! A crate for reading bitfields that are defined at runtime.

pub mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
