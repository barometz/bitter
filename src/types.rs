//! Types used to describe bitfields. These describe structure and do not hold any actual values.

/// A single bit field, representing an individual value
#[derive(Debug)]
pub struct Field {
    /// The name of this field
    pub name: String,
    /// The offset in bits from 0 at which this field starts
    pub offset: usize,
    /// The size in bits
    pub bits: usize,
}

impl Field {
    /// Create a new Field with a name but no data
    ///
    /// ```
    /// use bitview::Field;
    ///
    /// let field = Field::new("OSR");
    /// assert_eq!(field.name, String::from("OSR"));
    /// ```
    pub fn new(name: &str) -> Field {
        Field {
            name: name.into(),
            offset: 0,
            bits: 0,
        }
    }

    /// Give a field a size
    ///
    /// ```
    /// use bitview::Field;
    ///
    /// let field = Field::new("OSR").downto(14, 7);
    /// assert_eq!(field.offset, 7);
    /// assert_eq!(field.bits, 8);
    /// ```
    pub fn downto(self, top: usize, bottom: usize) -> Field {
        Field {
            offset: bottom,
            bits: top - bottom + 1,
            ..self
        }
    }
}

/// A structure of bit fields
pub struct Structure {
    /// The name of the structure
    pub name: String,
    /// The fields that make up the structure.
    ///
    /// The fields don't have to be contiguous, but the names must be unique.
    pub fields: Vec<Field>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
