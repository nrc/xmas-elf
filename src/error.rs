use core::fmt;

/// Errors returned by the methods and functions of this crate.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    /// The magic number of the given ELF file is invalid.
    InvalidMagic,
    /// The class of the given ELF file is invalid.
    InvalidClass,
    /// The section type is invalid.
    InvalidSectionType,
    /// The segment type is invalid.
    InvalidSegmentType,
    /// The version of the given ELF file is invalid.
    InvalidVersion,
    /// The data format of the given ELF file is invalid.
    InvalidDataFormat,
    /// The symbol's binding is invalid.
    InvalidSymbolBinding,
    /// The length of the given ELF file is too short.
    FileIsTooShort,
    /// Program header is not found.
    ProgramHeaderNotFound,
    /// The `.symtab_shndx` section is not found.
    SymtabShndxNotFound,
    /// The `.strtab` section is not found.
    StrtabNotFound,
    /// The `.dynstr` section is not found.
    DynstrNotFound,
    /// The section type is `NULL`.
    SectionIsNull,
    /// The section header index is one of the followings:
    /// - `SHN_UNDEF`
    /// - `SHN_ABS`
    /// - `SHN_COMMON`
    SectionHeaderIndexIsReserved,
    /// The size of each program header recorded in the file header is different from the actual
    /// size.
    ProgramHeaderSizeMismatch,
    /// The class specified in the file header is different from the actual class.
    ClassMismatch,
    /// The segment whose type is `PT_SHLIB` should not be used.
    UseOfShLib,
    /// The alignments of the virtual address, offset, and align recorded in the program header are
    /// the invalid combination.
    MisalignedAddressAndOffset,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidMagic => "The magic number of the given ELF file is invalid.",
                Self::InvalidClass => "The class of the given ELF file is invalid.",
                Self::InvalidSectionType => "The section type is invalid.",
                Self::InvalidSegmentType => "The segment type is invalid.",
                Self::InvalidVersion => "The version of the given ELF file is invalid.",
                Self::InvalidDataFormat => "The data format of the given ELF file is invalid.",
                Self::InvalidSymbolBinding => "The symbol's binding is invalid.",
                Self::FileIsTooShort => "The length of the given ELF file is too short.",
                Self::ProgramHeaderNotFound => "The program header is not found.",
                Self::SymtabShndxNotFound => "The `.symtab_shndx` section is not found.",
                Self::StrtabNotFound => "The `.strtab` section is not found.",
                Self::DynstrNotFound => "The `.dynstr` section is not found.",
                Self::SectionIsNull => "The section type is `NULL`.",
                Self::SectionHeaderIndexIsReserved => "The section header index is reserved.",
                Self::ProgramHeaderSizeMismatch => "The size of each program header recorded in the file header is different from the actual size.",
                Self::ClassMismatch => "The class specified in the file header is different from the actual class.",
                Self::UseOfShLib => "The segment whose type is `PT_SHLIB` should not be used.",
                Self::MisalignedAddressAndOffset => "The alignments of the virtual address, offset, and align recorded in the program header are the invalid combination.",
            }
        )
    }
}
