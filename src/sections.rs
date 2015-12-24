use std::fmt;

use {P32, P64};
use header::{Header, Class};
use parsing::{parse_one, parse_str};

// TODO iterate over section headers
// TODO sanity_check

pub fn parse_section_header<'a>(input: &'a [u8],
                                header: Header<'a>,
                                index: u16) -> SectionHeader<'a> {
    // Trying to get index 0 (SHN_UNDEF) is also probably an error, but it is a legitimate section.
    assert!(index < SHN_LORESERVE, "Attempt to get section for a reserved index");

    let start = (index as u64 * header.pt2.sh_entry_size() as u64 + header.pt2.sh_offset() as u64) as usize;
    let end = start + header.pt2.sh_entry_size() as usize;

    match header.pt1.class {
        Class::ThirtyTwo => {
            let header: &'a SectionHeader_<P32> = parse_one(&input[start..end]);
            SectionHeader::Sh32(header)
        }
        Class::SixtyFour => {
            let header: &'a SectionHeader_<P64> = parse_one(&input[start..end]);
            SectionHeader::Sh64(header)
        }
        Class::None => unreachable!(),
    }
}

// Distinguished section indices.
pub const SHN_UNDEF: u16        = 0;
pub const SHN_LORESERVE: u16    = 0xff00;
pub const SHN_LOPROC: u16       = 0xff00;
pub const SHN_HIPROC: u16       = 0xff1f;
pub const SHN_LOOS: u16         = 0xff20;
pub const SHN_HIOS: u16         = 0xff3f;
pub const SHN_ABS: u16          = 0xfff1;
pub const SHN_COMMON: u16       = 0xfff2;
pub const SHN_XINDEX: u16       = 0xffff;
pub const SHN_HIRESERVE: u16    = 0xffff;

#[derive(Clone, Copy)]
pub enum SectionHeader<'a> {
    Sh32(&'a SectionHeader_<P32>),
    Sh64(&'a SectionHeader_<P64>),
}

macro_rules! getter {
    ($name: ident, $typ: ident) => {
        pub fn $name(&self) -> $typ {
            match *self {
                SectionHeader::Sh32(h) => h.$name as $typ,
                SectionHeader::Sh64(h) => h.$name as $typ,
            }        
        }
    }
}

impl<'a> SectionHeader<'a> {
    // Note that this function is O(n) in the length of the name.
    pub fn name_as_str(&self, input: &'a [u8], header: Header<'a>) -> &'a str {
        // TODO move this to a method on header (and cache it eventually).
        let header = parse_section_header(input, header, header.pt2.sh_str_index());
        parse_str(&input[header.offset() as usize], self.name() as usize)
    }

    getter!(name, u32);
    getter!(offset, u64);
}

impl<'a> fmt::Display for SectionHeader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SectionHeader::Sh32(sh) => sh.fmt(f),
            SectionHeader::Sh64(sh) => sh.fmt(f),
        }
    }
}

impl<P: fmt::Debug> fmt::Display for SectionHeader_<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Section header:"));
        try!(writeln!(f, "    name:             {:?}", self.name));
        try!(writeln!(f, "    type:             {:?}", self.type_));
        try!(writeln!(f, "    flags:            {:?}", self.flags));
        try!(writeln!(f, "    address:          {:?}", self.address));
        try!(writeln!(f, "    offset:           {:?}", self.offset));
        try!(writeln!(f, "    size:             {:?}", self.size));
        try!(writeln!(f, "    link:             {:?}", self.link));
        try!(writeln!(f, "    align:            {:?}", self.align));
        try!(writeln!(f, "    entry size:       {:?}", self.entry_size));
        Ok(())
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SectionHeader_<P> {
    name: u32,
    type_: ShType_,
    flags: P,
    address: P,
    offset: P,
    size: P,
    link: u32,
    info: u32,
    align: P,
    entry_size: P,
}

pub struct ShType_(u32);

#[derive(Debug)]
pub enum ShType {
    Null,
    ProgBits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    ShLib,
    DynSym,
    InitArray,
    FiniArray,
    PreInitArray,
    Group,
    SymTabShIndex,
    OsSpecific(u32),
    ProcessorSpecific(u32),
    User(u32),
}

impl ShType_ {
    fn as_sh_type(&self) -> ShType {
        match self.0 {
            0 => ShType::Null,
            1 => ShType::ProgBits,
            2 => ShType::SymTab,
            3 => ShType::StrTab,
            4 => ShType::Rela,
            5 => ShType::Hash,
            6 => ShType::Dynamic,
            7 => ShType::Note,
            8 => ShType::NoBits,
            9 => ShType::Rel,
            10 => ShType::ShLib,
            11 => ShType::DynSym,
            // sic.
            14 => ShType::InitArray,
            15 => ShType::FiniArray,
            16 => ShType::PreInitArray,
            17 => ShType::Group,
            18 => ShType::SymTabShIndex,
            st if st >= SHT_LOOS && st <= SHT_HIOS => ShType::OsSpecific(st),
            st if st >= SHT_LOPROC && st <= SHT_HIPROC => ShType::ProcessorSpecific(st),
            st if st >= SHT_LOUSER && st <= SHT_HIUSER => ShType::User(st),
            _ => panic!("Invalid sh type"),
        }
    }
}

impl fmt::Debug for ShType_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_sh_type().fmt(f)
    }
}

// Distinguished ShType values.
pub const SHT_LOOS: u32   = 0x60000000;
pub const SHT_HIOS: u32   = 0x6fffffff;
pub const SHT_LOPROC: u32 = 0x70000000;
pub const SHT_HIPROC: u32 = 0x7fffffff;
pub const SHT_LOUSER: u32 = 0x80000000;
pub const SHT_HIUSER: u32 = 0xffffffff;

// Flags (SectionHeader::flags)
pub const SHF_WRITE: u64            =        0x1;
pub const SHF_ALLOC: u64            =        0x2;
pub const SHF_EXECINSTR: u64        =        0x4;
pub const SHF_MERGE: u64            =       0x10;
pub const SHF_STRINGS: u64          =       0x20;
pub const SHF_INFO_LINK: u64        =       0x40;
pub const SHF_LINK_ORDER: u64       =       0x80;
pub const SHF_OS_NONCONFORMING: u64 =      0x100;
pub const SHF_GROUP: u64            =      0x200;
pub const SHF_TLS: u64              =      0x400;
pub const SHF_COMPRESSED: u64       =      0x800;
pub const SHF_MASKOS: u64           = 0x0ff00000;
pub const SHF_MASKPROC: u64         = 0xf0000000;

#[derive(Debug)]
#[repr(C)]
pub struct CompressionHeader64 {
    type_: CompressionType_,
    _reserved: u32,
    size: u64,
    align: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct CompressionHeader32 {
    type_: CompressionType_,
    size: u32,
    align: u32,
}

pub struct CompressionType_(u32);

#[derive(Debug)]
pub enum CompressionType {
    Zlib,
    OsSpecific(u32),
    ProcessorSpecific(u32),
}

impl CompressionType_ {
    fn as_compression_type(&self) -> CompressionType {
        match self.0 {
            1 => CompressionType::Zlib,
            st if st >= COMPRESS_LOOS && st <= COMPRESS_HIOS => CompressionType::OsSpecific(st),
            st if st >= COMPRESS_LOPROC && st <= COMPRESS_HIPROC => CompressionType::ProcessorSpecific(st),
            _ => panic!("Invalid sh type"),
        }
    }
}

impl fmt::Debug for CompressionType_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_compression_type().fmt(f)
    }
}

// Distinguished CompressionType values.
pub const COMPRESS_LOOS: u32   = 0x60000000;
pub const COMPRESS_HIOS: u32   = 0x6fffffff;
pub const COMPRESS_LOPROC: u32 = 0x70000000;
pub const COMPRESS_HIPROC: u32 = 0x7fffffff;

// TODO SHT_GROUP section

// Group flags
pub const GRP_COMDAT: u64   =        0x1;
pub const GRP_MASKOS: u64   = 0x0ff00000;
pub const GRP_MASKPROC: u64 = 0xf0000000;
