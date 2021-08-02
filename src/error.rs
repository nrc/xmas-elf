use core::fmt;

/// Errors returned by the methods and functions of this crate.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    /// The magic number of the given ELF file is invalid.
    InvalidMagic,
    /// The class of the given ELF file is invalid.
    InvalidClass,
    /// The length of the given ELF file is too short.
    FileIsTooShort,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidMagic => "The magic number of the given ELF file is invalid.",
                Self::InvalidClass => "The class of the given ELF file is invalid.",
                Self::FileIsTooShort => "The length of the given ELF file is too short.",
            }
        )
    }
}
