//! Represent values.

use types;

fn mask(bits: usize) -> usize {
    2_usize.pow(bits as u32) - 1
}

/// A field value.
pub enum FieldValue {
    /// A boolean value, corresponding to `Field::Boolean`
    Boolean(bool),
    /// An integer value, corresponding to `Field::Integer`
    Integer(usize),
    /// An enumeration value, corresponding to `Field::Enum`
    Enum(usize, String),
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

    /// Get the unsigned integer value of any named field
    pub fn get_as_integer(&self, name: &str) -> Option<usize> {
        let range = self.structure.get_range(name);
        range.map(|(high, low)| {
            let width = high - low + 1;
            let shifted = self.data >> low;
            shifted & mask(width)
        })
    }

    /// Get an integer value for the given field name, if it exists
    ///
    /// ```
    /// use bitview::{Structure, Field, Value};
    ///
    /// let val = Value::new(
    ///     0xFFE5,
    ///     Structure::new("reg", &[
    ///         Field::integer("low_nibble_a", 4),
    ///         Field::integer("high_nibble_a", 4),
    ///         Field::integer("high_byte_b", 8),
    ///     ]));
    ///
    /// assert_eq!(val.get_integer("high_nibble_a"), Some(0xE));
    /// ```
    pub fn get_integer(&self, name: &str) -> Option<usize> {
        self.get_as_integer(name)
    }

    /// Get a boolean value for the given field name, if it exists.
    pub fn get_bool(&self, name: &str) -> Option<bool> {
        let raw = self.get_as_integer(name);
        raw.map(|integer| integer == 1)
    }
}
