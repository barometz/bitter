//! Represent values.

use types;

fn mask(bits: usize) -> usize {
    2_usize.pow(bits as u32) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
