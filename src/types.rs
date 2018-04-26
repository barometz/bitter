//! Types used to describe bitfields. These describe structure and do not hold any actual values.

use std::collections::HashMap;

/// The size of a single bit field.
pub type FieldSize = usize;
/// The name of a bit field.
pub type FieldName = String;
/// A mapping between enum values and descriptions
pub type EnumMapping = HashMap<usize, String>;

/// A bit field.
#[derive(Clone, Debug)]
pub enum Field {
    /// Reserved/unused field with the given width
    Reserved(FieldSize),
    /// One-bit boolean - 1 is true, 0 is false.
    Boolean(FieldName),
    /// An unsigned integer field with the given width.
    Integer(FieldName, FieldSize),
    /// An enumeration with a particular size and value-name mapping.
    Enum(FieldName, FieldSize, EnumMapping),
}

impl Field {
    /// Create a new Field::Reserved.
    pub fn reserved(size: FieldSize) -> Field {
        Field::Reserved(size)
    }

    /// Create a new Field::Boolean
    pub fn boolean(name: &str) -> Field {
        Field::Boolean(name.into())
    }

    /// Create a new Field::Integer
    pub fn integer(name: &str, size: FieldSize) -> Field {
        Field::Integer(name.into(), size)
    }

    /// Create a new Field::Enum
    pub fn enumeration(name: &str, size: FieldSize, map: EnumMapping) -> Field {
        Field::Enum(name.into(), size, map)
    }

    /// Get the size in bits.
    ///
    /// ```
    /// use bitter::Field;
    /// let res_act : Field = Field::Boolean("res_act".into());
    /// assert_eq!(res_act.size(), 1);
    /// ```
    pub fn size(&self) -> FieldSize {
        match *self {
            Field::Reserved(n) => n,
            Field::Boolean(_) => 1,
            Field::Integer(_, n) => n,
            Field::Enum(_, n, _) => n,
        }
    }

    /// Get the name of the field, unless it's Field::Reserved
    pub fn get_name(&self) -> Option<String> {
        match *self {
            Field::Reserved(_) => None,
            Field::Boolean(ref name)
            | Field::Integer(ref name, _)
            | Field::Enum(ref name, _, _) => Some(name.clone())
        }
    }
}

/// A type made up of bit fields.
///
/// ```
/// use bitter::{Structure, Field};
///
/// let reg = Structure::new(
///    "REG_1",
///    &[
///        Field::boolean("bob"),
///        Field::reserved(4),
///        Field::integer("jim", 3)
///    ],
/// );
/// ```
#[derive(Debug)]
pub struct Structure {
    /// The name of the structure. Generally the name of the represented register.
    pub name: String,
    /// List of components, starting with the most significant bits
    pub fields: Vec<Field>,
}

impl Structure {
    /// Create a new Structure with the given name.
    pub fn new(name: &str, fields: &[Field]) -> Structure {
        Structure {
            name: name.into(),
            fields: fields.into(),
        }
    }

    /// Append a field to an existing Structure. The new field becomes the least-significant one.
    pub fn append(&mut self, field: Field) {
        self.fields.push(field);
    }

    /// Get the size of all the fields combined. This is distinct from the size of the (integer)
    /// value the structure is created from.
    ///
    /// ```
    /// use bitter::{Structure, Field};
    ///
    /// let mut reg = Structure::new("reg", &[]);
    /// reg.fields.push(Field::Boolean("res_act".into()));
    /// reg.fields.push(Field::Reserved(4));
    ///
    /// assert_eq!(reg.size(), 5);
    /// ```
    pub fn size(&self) -> FieldSize {
        self.fields.iter().fold(0, |acc, field| acc + field.size())
    }

    /// Get the range of bits for the specified field, if it exists.
    ///
    /// The resulting range is in downto notation, so (3, 0) means the four least significant bits.
    ///
    /// # Example
    /// ```
    /// use bitter::{Structure, Field};
    ///
    /// let reg = Structure::new("reg", &[
    ///     Field::integer("lifetime", 4),
    ///     Field::boolean("active"),
    /// ]);
    ///
    /// assert_eq!(reg.get_range("lifetime"), Some((4, 1)));
    /// ```
    pub fn get_range(&self, field_name: &str) -> Option<(usize, usize)> {
        let mut low : FieldSize = 0;
        for field in self.fields.iter().rev() {
            if let Some(name) = field.get_name() {
                if name == field_name {
                    return Some((low + field.size() - 1, low))
                }
            }

            low = low + field.size()
        }

        None
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

    #[test]
    fn structure_empty() {
        assert_eq!(Structure::new("Any", &[]).size(), 0);
    }

    #[test]
    fn structure_with_fields() {
        assert_eq!(
            8,
            Structure::new(
                "Any",
                &[
                    Field::boolean("bob"),
                    Field::reserved(4),
                    Field::integer("jim", 3)
                ],
            ).size()
        )
    }
}
