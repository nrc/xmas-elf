use ElfFile;
use sections;

use std::fmt;
use std::mem;

#[derive(Debug)]
#[repr(C)]
pub struct Entry32 {
    name: u32,
    value: u32,
    size: u32,
    info: u8,
    other: Visibility_,
    shndx: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct Entry64 {
    name: u32,
    info: u8,
    other: Visibility_,
    shndx: u16,
    value: u64,
    size: u64,
}

pub trait Entry {
    fn name(&self) -> u32;
    fn info(&self) -> u8;
    fn other(&self) -> Visibility_;
    fn shndx(&self) -> u16;
    fn value(&self) -> u64;
    fn size(&self) -> u64;

    // Note that this function is O(n) in the length of the name.
    fn get_name<'a>(&'a self, elf_file: &ElfFile<'a>) -> &'a str {
        elf_file.get_string(self.name())
    }

    fn get_other(&self) -> Visibility {
        self.other().as_visibility()
    }

    fn get_binding(&self) -> Binding {
        Binding_(self.info() >> 4).as_binding()
    }

    fn get_type(&self) -> Type {
        Type_(self.info() & 0xf).as_type()
    }

    fn get_section_header<'a>(&'a self,
                              elf_file: &ElfFile<'a>,
                              self_index: usize)
                              -> Option<sections::SectionHeader<'a>> {
        match self.shndx() {
            sections::SHN_XINDEX => {
                // TODO factor out distinguished section names into sections consts
                let header = elf_file.find_section_by_name(".symtab_shndx");
                if let Some(header) = header {
                    assert!(header.get_type() == sections::ShType::SymTabShIndex);
                    if let sections::SectionData::SymTabShIndex(data) = header.get_data(elf_file) {
                        // TODO cope with u32 section indices (count is in sh_size of header 0, etc.)
                        // Note that it is completely bogus to crop to u16 here.
                        let index = data[self_index] as u16;
                        assert!(index != sections::SHN_UNDEF);
                        Some(sections::parse_section_header(elf_file.input, elf_file.header, index))
                    } else {
                        panic!("Expected SymTabShIndex");
                    }
                } else {
                    panic!("no .symtab_shndx section");
                }
            }
            sections::SHN_UNDEF | sections::SHN_ABS | sections::SHN_COMMON => None,
            i => Some(sections::parse_section_header(elf_file.input, elf_file.header, i)),
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "Symbol table entry:"));
        try!(writeln!(f, "    name:             {:?}", self.name()));
        try!(writeln!(f, "    binding:          {:?}", self.get_binding()));
        try!(writeln!(f, "    type:             {:?}", self.get_type()));
        try!(writeln!(f, "    other:            {:?}", self.get_other()));
        try!(writeln!(f, "    shndx:            {:?}", self.shndx()));
        try!(writeln!(f, "    value:            {:?}", self.value()));
        try!(writeln!(f, "    size:             {:?}", self.size()));
        Ok(())
    }
}

macro_rules! impl_entry {
    ($name: ident) => {
        impl Entry for $name {
            fn name(&self) -> u32 { self.name }
            fn info(&self) -> u8 { self.info }
            fn other(&self) -> Visibility_ { self.other }
            fn shndx(&self) -> u16 { self.shndx }
            fn value(&self) -> u64 { self.value as u64 }
            fn size(&self) -> u64 { self.size as u64 }
        }
    }
}
impl_entry!(Entry32);
impl_entry!(Entry64);

impl fmt::Display for Entry32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Entry::fmt(self, f)
    }
}
impl fmt::Display for Entry64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Entry::fmt(self, f)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Visibility_(u8);

#[derive(Debug)]
#[repr(u8)]
pub enum Visibility {
    Default = 0,
    Internal = 1,
    Hidden = 2,
    Protected = 3,
}

impl Visibility_ {
    pub fn as_visibility(self) -> Visibility {
        unsafe { mem::transmute(self.0 & 0x3) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Binding_(u8);

#[derive(Debug)]
pub enum Binding {
    Local,
    Global,
    Weak,
    OsSpecific(u8),
    ProcessorSpecific(u8),
}

impl Binding_ {
    pub fn as_binding(self) -> Binding {
        match self.0 {
            0 => Binding::Local,
            1 => Binding::Global,
            2 => Binding::Weak,
            b if b >= 10 && b <= 12 => Binding::OsSpecific(b),
            b if b >= 13 && b <= 15 => Binding::ProcessorSpecific(b),
            _ => panic!("Invalid value for binding"),
        }
    }
}

// TODO should use a procedural macro for generating these things
#[derive(Copy, Clone, Debug)]
pub struct Type_(u8);

#[derive(Debug)]
pub enum Type {
    NoType,
    Object,
    Func,
    Section,
    File,
    Common,
    Tls,
    OsSpecific(u8),
    ProcessorSpecific(u8),
}

impl Type_ {
    pub fn as_type(self) -> Type {
        match self.0 {
            0 => Type::NoType,
            1 => Type::Object,
            2 => Type::Func,
            3 => Type::Section,
            4 => Type::File,
            5 => Type::Common,
            6 => Type::Tls,
            b if b >= 10 && b <= 12 => Type::OsSpecific(b),
            b if b >= 13 && b <= 15 => Type::ProcessorSpecific(b),
            _ => panic!("Invalid value for type"),
        }
    }
}
