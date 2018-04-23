//! Types used to describe bitfields.

use std::collections::HashMap;

/// The size of a single bit field.
pub type FieldSize = usize;
/// The name of a bit field.
pub type FieldName = String;

/// A bit field.
#[derive(Debug)]
pub enum Field {
    /// Reserved/unused field with the given width
    Reserved(FieldSize),
    /// One-bit boolean - 1 is true, 0 is false.
    Boolean(FieldName),
    /// An unsigned integer field with the given width.
    Integer(FieldName, FieldSize),
    /// An enumeration with a particular size and value-name mapping.
    Enum(FieldName, FieldSize, HashMap<usize, String>),
}

impl Field {
    /// Get the size in bits.
    ///
    /// ```
    /// use bitter::Field;
    /// let res_act : Field = Field::Boolean("res_act".into());
    /// assert_eq!(res_act.size(), 1);
    /// ```
    pub fn size(&self) -> FieldSize {
        match self {
            &Field::Reserved(n) => n,
            &Field::Boolean(_) => 1,
            &Field::Integer(_, n) => n,
            &Field::Enum(_, n, _) => n,
        }
    }
}

/// A type made up of bit fields
#[derive(Debug)]
pub struct Structure {
    /// The name of the structure. Generally the name of the represented register.
    pub name: String,
    /// List of components, starting with the most significant bits
    pub fields: Vec<Field>,
}

impl Structure {
    /// Create a new Structure with the given name.
    pub fn new(name: &str) -> Structure {
        Structure { name: name.into(), fields: Vec::new() }
    }

    /// Get the size of all the fields combined. This is distinct from the size of the (integer)
    /// value the structure is created from.
    ///
    /// ```
    /// use bitter::{Structure, Field};
    ///
    /// let mut reg = Structure::new("reg");
    /// reg.fields.push(Field::Boolean("res_act".into()));
    /// reg.fields.push(Field::Reserved(4));
    ///
    /// assert_eq!(reg.size(), 5);
    /// ```
    pub fn size(&self) -> FieldSize {
        self.fields.iter().fold(0, |acc, field| acc + field.size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fieldsize_reserved() {
        assert_eq!(Field::Reserved(4).size(), 4);
    }

    #[test]
    fn fieldsize_bool() {
        assert_eq!(Field::Boolean("Any".into()).size(), 1);
    }

    #[test]
    fn fieldsize_integer() {
        assert_eq!(Field::Integer("Any".into(), 14).size(), 14);
    }

    #[test]
    fn fieldsize_enum() {
        assert_eq!(Field::Enum("Any".into(), 36, HashMap::new()).size(), 36);
    }
}
