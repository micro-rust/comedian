//! `comedian` is an attempt to have a cross-crate representation of architectural endianness.



#![forbid(unsafe_code)]



/// Enumeration of the two endian possibilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Endianness {
    /// Big endian representation.
    Big,

    /// Little endian representation.
    Little,
}

impl Default for Endianness {
    #[cfg(target_endian = "little")]
    #[inline]
    fn default() -> Endianness {
        Endianness::Little
    }

    #[cfg(target_endian = "big")]
    #[inline]
    fn default() -> Endianness {
        Endianness::Big
    }
}



/// Single struct type representing the big endianness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BigEndian;

impl BigEndian {
    /// Associated endianness enum value.
    pub const ENDIANNESS: Endianness = Endianness::Little;
}

impl Default for BigEndian {
    fn default() -> BigEndian {
        BigEndian
    }
}

impl core::convert::TryFrom<Endianness> for BigEndian {
    type Error = ();

    fn try_from(endian: Endianness) -> Result<BigEndian, ()> {
        match endian {
            Endianness::Big => Ok( BigEndian ),
            _ => Err(()),
        }
    }
}



/// Single struct type representing the little endianness.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LittleEndian;

impl LittleEndian {
    /// Associated endianness enum value.
    pub const ENDIANNESS: Endianness = Endianness::Little;
}

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        LittleEndian
    }
}

impl core::convert::TryFrom<Endianness> for LittleEndian {
    type Error = ();

    fn try_from(endian: Endianness) -> Result<LittleEndian, ()> {
        match endian {
            Endianness::Little => Ok( LittleEndian ),
            _ => Err(()),
        }
    }
}



/// Defines network byte endianness.
/// Defined by [RFC 1700][1] to be big-endian.
/// This type is an alias for `BigEndian`.
///
/// [1]: https://tools.ietf.org/html/rfc1700
pub type NetworkEndian = BigEndian; 



/// Defines the system's native-endian serialization.
/// This type is an alias for `BigEndian`.
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;



/// Defines the system's native-endian serialization.
/// This type is an alias for `LittleEndian`.
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;
