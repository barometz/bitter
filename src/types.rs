//! Types used to describe bitfields.

use std::collections::HashMap;

type FieldSize = usize;

/// Data size and type of a single bitfield component.
#[derive(Debug)]
pub enum ValueType {
    /// One-bit boolean - 1 is true, 0 is false.
    Boolean,
    /// An unsigned integer field with a particular size.
    Integer(FieldSize),
    /// An enumeration with a particular size and value-name mapping.
    Enum(FieldSize, HashMap<usize, String>)
}

/// A single component of the bitfield.
#[derive(Debug)]
pub struct Value {
    /// The name of the component.
    pub name: String,
    /// The type of the field.
    pub field_type: ValueType,
}

/// A value or reserved space. Reserved is separated since a reserved field has no name.
#[derive(Debug)]
pub enum Field {
    /// Reserved/unused field with the given width
    Reserved(FieldSize),
    /// A value
    Value(Value),
}

/// A type made up of bit fields
#[derive(Debug)]
pub struct Structure {
    /// The name of the structure. Generally the name of the represented register.
    pub name: String,
    /// List of components, starting with the most significant bits
    pub fields: Vec<Field>,
}
