use core::fmt;
use core::mem;

use {P32, P64, ElfFile};
use zero::{read, Pod};


pub fn parse_header<'a>(input: &'a [u8]) -> Header<'a> {
    let size_pt1 = mem::size_of::<HeaderPt1>();
    let header_1: &'a HeaderPt1 = read(&input[..size_pt1]);
    assert!(header_1.magic == MAGIC);

    let header_2 = match header_1.class {
        Class::None => Err("Invalid ELF class"),
        Class::ThirtyTwo => {
            let header_2: &'a HeaderPt2_<P32> =
                read(&input[size_pt1..size_pt1 + mem::size_of::<HeaderPt2_<P32>>()]);
            Ok(HeaderPt2::Header32(header_2))
        }
        Class::SixtyFour => {
            let header_2: &'a HeaderPt2_<P64> =
                read(&input[size_pt1..size_pt1 + mem::size_of::<HeaderPt2_<P64>>()]);
            Ok(HeaderPt2::Header64(header_2))
        }
    };
    Header {
        pt1: header_1,
        pt2: header_2,
    }
}

pub const MAGIC: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];

#[derive(Clone, Copy)]
pub struct Header<'a> {
    pub pt1: &'a HeaderPt1,
    pub pt2: Result<HeaderPt2<'a>, &'static str>,
}

// TODO add Header::section_count, because if sh_count = 0, then the real count is in the first section.

impl<'a> fmt::Display for Header<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "ELF header:"));
        try!(writeln!(f, "    magic:            {:?}", self.pt1.magic));
        try!(writeln!(f, "    class:            {:?}", self.pt1.class));
        try!(writeln!(f, "    data:             {:?}", self.pt1.data));
        try!(writeln!(f, "    version:          {:?}", self.pt1.version));
        try!(writeln!(f, "    os abi:           {:?}", self.pt1.os_abi));
        try!(writeln!(f, "    abi version:      {:?}", self.pt1.abi_version));
        try!(writeln!(f, "    padding:          {:?}", self.pt1.padding));
        try!(self.pt2.ok().map_or(Ok(()), |pt2| write!(f, "{}", pt2)));
        Ok(())
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct HeaderPt1 {
    pub magic: [u8; 4],
    pub class: Class,
    pub data: Data,
    pub version: Version,
    pub os_abi: OsAbi,
    // Often also just padding.
    pub abi_version: u8,
    pub padding: [u8; 7],
}

unsafe impl Pod for HeaderPt1 {}

#[derive(Clone, Copy)]
pub enum HeaderPt2<'a> {
    Header32(&'a HeaderPt2_<P32>),
    Header64(&'a HeaderPt2_<P64>),
}

macro_rules! getter {
    ($name: ident, $typ: ident) => {
        pub fn $name(&self) -> $typ {
            match *self {
                HeaderPt2::Header32(h) => h.$name as $typ,
                HeaderPt2::Header64(h) => h.$name as $typ,
            }
        }
    }
}

impl<'a> HeaderPt2<'a> {
    pub fn size(&self) -> usize {
        match *self {
            HeaderPt2::Header32(_) => mem::size_of::<HeaderPt2_<P32>>(),
            HeaderPt2::Header64(_) => mem::size_of::<HeaderPt2_<P64>>(),
        }
    }

    // TODO move to impl Header
    getter!(type_, Type_);
    getter!(machine, Machine);
    getter!(version, u32);
    getter!(header_size, u16);
    getter!(entry_point, u64);
    getter!(ph_offset, u64);
    getter!(sh_offset, u64);
    getter!(ph_entry_size, u16);
    getter!(ph_count, u16);
    getter!(sh_entry_size, u16);
    getter!(sh_count, u16);
    getter!(sh_str_index, u16);
}

impl<'a> fmt::Display for HeaderPt2<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HeaderPt2::Header32(h) => write!(f, "{}", h),
            HeaderPt2::Header64(h) => write!(f, "{}", h),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HeaderPt2_<P> {
    pub type_: Type_,
    pub machine: Machine,
    pub version: u32,
    pub entry_point: P,
    pub ph_offset: P,
    pub sh_offset: P,
    pub flags: u32,
    pub header_size: u16,
    pub ph_entry_size: u16,
    pub ph_count: u16,
    pub sh_entry_size: u16,
    pub sh_count: u16,
    pub sh_str_index: u16,
}

unsafe impl<P> Pod for HeaderPt2_<P> {}

impl<P: fmt::Display> fmt::Display for HeaderPt2_<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "    type:             {:?}", self.type_));
        try!(writeln!(f, "    machine:          {:?}", self.machine));
        try!(writeln!(f, "    version:          {}", self.version));
        try!(writeln!(f, "    entry_point:      {}", self.entry_point));
        try!(writeln!(f, "    ph_offset:        {}", self.ph_offset));
        try!(writeln!(f, "    sh_offset:        {}", self.sh_offset));
        try!(writeln!(f, "    flags:            {}", self.flags));
        try!(writeln!(f, "    header_size:      {}", self.header_size));
        try!(writeln!(f, "    ph_entry_size:    {}", self.ph_entry_size));
        try!(writeln!(f, "    ph_count:         {}", self.ph_count));
        try!(writeln!(f, "    sh_entry_size:    {}", self.sh_entry_size));
        try!(writeln!(f, "    sh_count:         {}", self.sh_count));
        try!(writeln!(f, "    sh_str_index:     {}", self.sh_str_index));
        Ok(())
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Class {
    None = 0,
    ThirtyTwo = 1,
    SixtyFour = 2,
}

impl Class {
    pub fn is_none(&self) -> bool {
        if let Class::None = *self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Data {
    None = 0,
    LittleEndian = 1,
    BigEndian = 2,
}

impl Data {
    pub fn is_none(&self) -> bool {
        if let Data::None = *self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Version {
    None = 0,
    Current = 1,
}

impl Version {
    pub fn is_none(&self) -> bool {
        if let Version::None = *self {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum OsAbi {
    // or None
    SystemV = 0x00,
    HpUx = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    Solaris = 0x06,
    Aix = 0x07,
    Irix = 0x08,
    FreeBSD = 0x09,
    OpenBSD = 0x0C,
    OpenVMS = 0x0D, // FIXME there are many, many more of these
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Type_(pub u16);

impl Type_ {
    pub fn as_type(self) -> Type {
        match self.0 {
            0 => Type::None,
            1 => Type::Relocatable,
            2 => Type::Executable,
            3 => Type::SharedObject,
            4 => Type::Core,
            x => Type::ProcessorSpecific(x),
        }
    }
}

impl fmt::Debug for Type_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_type().fmt(f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Type {
    None,
    Relocatable,
    Executable,
    SharedObject,
    Core,
    ProcessorSpecific(u16), // TODO OsSpecific
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum Machine {
    None = 0,
    Sparc = 0x02,
    X86 = 0x03,
    Mips = 0x08,
    PowerPC = 0x14,
    Arm = 0x28,
    SuperH = 0x2A,
    Ia64 = 0x32,
    X86_64 = 0x3E,
    AArch64 = 0xB7, // FIXME there are many, many more of these
}

// TODO any more constants that need to go in here?

pub fn sanity_check(file: &ElfFile) -> Result<(), &'static str> {
    check!(mem::size_of::<HeaderPt1>() == 16);
    check!(file.header.pt1.magic == MAGIC, "bad magic number");
    let pt2 = try!(file.header.pt2);
    check!(mem::size_of::<HeaderPt1>() + pt2.size() == pt2.header_size() as usize,
           "header_size does not match size of header");
    match (&file.header.pt1.class, &file.header.pt2) {
        (&Class::None, _) => return Err("No class"),
        (&Class::ThirtyTwo, &Ok(HeaderPt2::Header32(_))) |
        (&Class::SixtyFour, &Ok(HeaderPt2::Header64(_))) => {}
        _ => return Err("Mismatch between specified and actual class"),
    }
    check!(!file.header.pt1.version.is_none(), "no version");
    check!(!file.header.pt1.data.is_none(), "no data format");

    check!(pt2.entry_point() < file.input.len() as u64,
           "entry point out of range");
    check!(pt2.ph_offset() + (pt2.ph_entry_size() as u64) * (pt2.ph_count() as u64) <=
           file.input.len() as u64,
           "program header table out of range");
    check!(pt2.sh_offset() + (pt2.sh_entry_size() as u64) * (pt2.sh_count() as u64) <=
           file.input.len() as u64,
           "section header table out of range");

    // TODO check that SectionHeader_ is the same size as sh_entry_size, depending on class

    Ok(())
}
