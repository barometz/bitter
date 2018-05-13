//! Types used to describe bitfields. These describe structure and do not hold any actual values.

use std::collections::HashMap;

/// The size of a single bit field.
pub type FieldSize = usize;
/// The name of a bit field.
pub type FieldName = String;
/// A mapping between enum values and descriptions
pub type EnumMapping = HashMap<usize, String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
