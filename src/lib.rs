#![deny(missing_docs)]

//! A crate for reading bitfields that are defined at runtime.

mod types;
mod values;

pub use types::*;
pub use values::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
