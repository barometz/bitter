A crate for reading bitfields that are defined at runtime.

The primary use case is to avoid having to write a full protocol analyzer for every new protocol and system you're dealing with. Bitview facilitates rapidly describing the layout of a structure, and then plugging numbers in to get the corresponding field values.

It is not meant to replace the various bitfield crates that support compile-time definitions. Because of its focus on runtime flexibility, performance will probably never be great and compile-time verification is minimal.
