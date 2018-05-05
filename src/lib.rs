#![deny(missing_docs)]

//! A crate for reading bitfields that are defined at runtime.
//!
//! The primary use case is to avoid having to write a full protocol analyzer for every new protocol
//! and system you're dealing with. Bitview facilitates rapidly describing the layout of a
//! structure, and then plugging numbers in to get the corresponding field values.
//!
//! It is not meant to replace the various bitfield crates that support compile-time definitions.
//! Because of its focus on runtime flexibility, performance will probably never be great and
//! compile-time verification is minimal.
//!
//! # Examples
//! TODO
//!
//! # Conventions
//! ## Names
//! Because this crate can be used to describe structure as well as read values, the naming gets
//! confusing if you're not careful. A *register* could be "the storage space at address 0x0700",
//! "the 32-bit value stored at 0x0700", or "the structure of the data at 0x0700" depending on
//! context.
//!
//! Layout types:
//!
//! - The [Field] enumeration is used to describe the kinds of values a single field can represent.
//! - The [Structure] structure is composed of multiple [Field]s to describe the internal layout of
//!   a complex value.
//!
//! Values:
//!
//! - The [Value] structure is composed of a [Structure] and an actual source value, and can be used
//!   to extract field values.
//!
//! ## Ordering
//! - Each [Structure] holds a list of [Field]s. The first element is the least-significant field.
//!   This is backwards from how C structures are commonly laid out, but more convenient to do math
//!   with because...
//! - No effort is made to deal with endianness at this time, which almost certainly means there is
//!   an implicit assumption of [little-endianness][wiki-le] in several components.
//!
//! [Value]: struct.Value.html
//! [Structure]: struct.Structure.html
//! [Field]: enum.Field.html
//! [wiki-le]: https://en.wikipedia.org/wiki/Endianness#Little-endian

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
