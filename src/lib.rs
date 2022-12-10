//! `comedian` is an attempt to have a cross-crate representation of architectural endianness.



/// Enumeration of the two endian possibilities.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Endian {
    /// Big endian representation.
    Big,

    /// Little endian representation.
    Little,
}

impl Default for Endian {
    #[cfg(target_endian = "little")]
    #[inline]
    fn default() -> Endian {
        Endian::Little
    }

    #[cfg(target_endian = "big")]
    #[inline]
    fn default() -> Endian {
        Endian::Big
    }
}



/// Single struct type representing the big endianness.
#[derive(Clone, Copt, Debug, Eq, PartialEq, Hash)]
pub struct BigEndian;

impl Default for BigEndian {
    fn default() -> BigEndian {
        BigEndian
    }
}

impl TryFrom<Endian> for BigEndian {
    fn try_from(endian: Endian) -> Result<BigEndian, ()> {
        match endian {
            Endian::Big => Ok( BigEndian ),
            _ => Err(()),
        }
    }
}



/// Single struct type representing the little endianness.
#[derive(Clone, Copt, Debug, Eq, PartialEq, Hash)]
pub struct LittleEndian;

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        LittleEndian
    }
}

impl TryFrom<Endian> for LittleEndian {
    fn try_from(endian: Endian) -> Result<LittleEndian, ()> {
        match endian {
            Endian::Little => Ok( LittleEndian ),
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
