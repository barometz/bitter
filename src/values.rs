//! Represent values.

use types;

fn mask(bits: usize) -> usize {
    2_usize.pow(bits as u32) - 1
}

/// A value, based on an integer and the structure of the bit fields within it.
pub struct Value {
    /// The integer data from which the individual bitfield values are derived.
    pub data: usize,
    /// The structure of the data.
    pub structure: types::Structure,
}

impl Value {
    /// Create a new `Value`
    pub fn new(data: usize, structure: types::Structure) -> Value {
        Value { data, structure }
    }

    /// Get an integer value for the given field name, if it exists
    ///
    /// ```
    /// use bitview::{Structure, Field, Value};
    ///
    /// let val = Value::new(
    ///     0xFFE5,
    ///     Structure::new("reg", &[
    ///         Field::integer("high_byte_b", 8),
    ///         Field::integer("high_nibble_a", 4),
    ///         Field::integer("low_nibble_a", 4),
    ///     ]));
    ///
    /// assert_eq!(val.get_integer("high_nibble_a"), Some(0xE));
    /// ```
    pub fn get_integer(&self, name: &str) -> Option<usize> {
        let range = self.structure.get_range(name);
        range.map(|(high, low)| {
            let shifted = self.data >> low;
            shifted & mask(high - low + 1)
        })
    }
}
