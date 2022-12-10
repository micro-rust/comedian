//! `comedian` is an attempt to have a cross-crate representation of architectural endianness.



/// Enumeration of the two endian possibilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    /// Big endian representation.
    Big,

    /// Little endian representation.
    Little,
}



/// Single struct type representing the big endianness.
#[derive(Clone, Copt, Debug, Eq, PartialEq)]
pub struct BigEndian;



/// Single struct type representing the little endianness.
#[derive(Clone, Copt, Debug, Eq, PartialEq)]
pub struct LittleEndian;
