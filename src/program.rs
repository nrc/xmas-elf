use {ElfFile, P32, P64};
use zero::{read, read_array, Pod};
use header::{Class, Header};
use dynamic::Dynamic;
use sections::NoteHeader;

use std::mem;
use std::fmt;


pub fn parse_program_header<'a>(input: &'a [u8],
                                header: Header<'a>,
                                index: u16)
                                -> ProgramHeader<'a> {
    assert!(index < header.pt2.ph_count() &&
            header.pt2.ph_offset() > 0 &&
            header.pt2.ph_entry_size() > 0);
    let start = header.pt2.ph_offset() as usize +
                index as usize * header.pt2.ph_entry_size() as usize;
    let end = start + header.pt2.ph_entry_size() as usize;

    match header.pt1.class {
        Class::ThirtyTwo => {
            let header: &'a ProgramHeader32 = read(&input[start..end]);
            ProgramHeader::Ph32(header)
        }
        Class::SixtyFour => {
            let header: &'a ProgramHeader64 = read(&input[start..end]);
            ProgramHeader::Ph64(header)
        }
        Class::None => unreachable!(),
    }
}

pub struct ProgramIter<'b, 'a: 'b> {
    pub file: &'b ElfFile<'a>,
    pub next_index: u16,
}

impl<'b, 'a> Iterator for ProgramIter<'b, 'a> {
    type Item = ProgramHeader<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.file.header.pt2.ph_count();
        if self.next_index >= count {
            return None;
        }

        let result = Some(self.file.program_header(self.next_index));
        self.next_index += 1;
        result
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ProgramHeader<'a> {
    Ph32(&'a ProgramHeader32),
    Ph64(&'a ProgramHeader64),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ProgramHeader32 {
    type_: Type_,
    offset: u32,
    virtual_addr: u32,
    physical_addr: u32,
    file_size: u32,
    mem_size: u32,
    flags: u32,
    align: u32,
}

unsafe impl Pod for ProgramHeader32 {}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ProgramHeader64 {
    type_: Type_,
    flags: u32,
    offset: u64,
    virtual_addr: u64,
    physical_addr: u64,
    file_size: u64,
    mem_size: u64,
    align: u64,
}

unsafe impl Pod for ProgramHeader64 {}

impl<'a> ProgramHeader<'a> {
    pub fn get_type(&self) -> Type {
        match *self {
            ProgramHeader::Ph32(ph) => ph.get_type(),
            ProgramHeader::Ph64(ph) => ph.get_type(),
        }
    }

    pub fn get_data(&self, elf_file: &ElfFile<'a>) -> SegmentData<'a> {
        match *self {
            ProgramHeader::Ph32(ph) => ph.get_data(elf_file),
            ProgramHeader::Ph64(ph) => ph.get_data(elf_file),
        }
    }
}

impl<'a> fmt::Display for ProgramHeader<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProgramHeader::Ph32(ph) => ph.fmt(f),
            ProgramHeader::Ph64(ph) => ph.fmt(f),
        }
    }
}
macro_rules! ph_impl {
    ($ph: ident) => {
        impl $ph {
            pub fn get_type(&self) -> Type {
                self.type_.as_type()
            }

            pub fn get_data<'a>(&self, elf_file: &ElfFile<'a>) -> SegmentData<'a> {
                match self.get_type() {
                    Type::Null => SegmentData::Empty,
                    Type::Load | Type::Interp | Type::ShLib | Type::Phdr | Type::Tls |
                    Type::OsSpecific(_) | Type::ProcessorSpecific(_) => {
                        SegmentData::Undefined(self.raw_data(elf_file))
                    }
                    Type::Dynamic => {
                        let data = self.raw_data(elf_file);
                        match elf_file.header.pt1.class {
                            Class::ThirtyTwo => SegmentData::Dynamic32(read_array(data)),
                            Class::SixtyFour => SegmentData::Dynamic64(read_array(data)),
                            Class::None => unreachable!(),
                        }
                    }
                    Type::Note => {
                        let data = self.raw_data(elf_file);
                        match elf_file.header.pt1.class {
                            Class::ThirtyTwo => unimplemented!(),
                            Class::SixtyFour => {
                                let header: &'a NoteHeader = read(&data[0..12]);
                                let index = &data[12..];
                                SegmentData::Note64(header, index)
                            }
                            Class::None => unreachable!(),
                        }
                    }
                }
            }

            pub fn raw_data<'a>(&self, elf_file: &ElfFile<'a>) -> &'a [u8] {
                assert!(self.get_type() != Type::Null);
                &elf_file.input[self.offset as usize..(self.offset + self.file_size) as usize]
            }
        }

        impl fmt::Display for $ph {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                try!(writeln!(f, "Program header:"));
                try!(writeln!(f, "    type:             {:?}", self.get_type()));
                try!(writeln!(f, "    flags:            {:?}", self.flags));
                try!(writeln!(f, "    offset:           {:?}", self.offset));
                try!(writeln!(f, "    virtual address:  {:?}", self.virtual_addr));
                try!(writeln!(f, "    physical address: {:?}", self.physical_addr));
                try!(writeln!(f, "    file size:        {:?}", self.file_size));
                try!(writeln!(f, "    memory size:      {:?}", self.mem_size));
                try!(writeln!(f, "    align:            {:?}", self.align));
                Ok(())
            }
        }
    }
}

ph_impl!(ProgramHeader32);
ph_impl!(ProgramHeader64);

#[derive(Copy, Clone)]
pub struct Type_(u32);

#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    ShLib,
    Phdr,
    Tls,
    OsSpecific(u32),
    ProcessorSpecific(u32),
}

impl Type_ {
    fn as_type(&self) -> Type {
        match self.0 {
            0 => Type::Null,
            1 => Type::Load,
            2 => Type::Dynamic,
            3 => Type::Interp,
            4 => Type::Note,
            5 => Type::ShLib,
            6 => Type::Phdr,
            7 => Type::Tls,
            t if t >= TYPE_LOOS && t <= TYPE_HIOS => Type::OsSpecific(t),
            t if t >= TYPE_LOPROC && t <= TYPE_HIPROC => Type::ProcessorSpecific(t),
            _ => panic!("Invalid type"),
        }
    }
}

impl fmt::Debug for Type_ {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_type().fmt(f)
    }
}

pub enum SegmentData<'a> {
    Empty,
    Undefined(&'a [u8]),
    Dynamic32(&'a [Dynamic<P32>]),
    Dynamic64(&'a [Dynamic<P64>]),
    // Note32 uses 4-byte words, which I'm not sure how to manage.
    // The pointer is to the start of the name field in the note.
    Note64(&'a NoteHeader, &'a [u8]),
    // TODO Interp and Phdr should probably be defined some how, but I can't find the details.
}

pub const TYPE_LOOS: u32   = 0x60000000;
pub const TYPE_HIOS: u32   = 0x6fffffff;
pub const TYPE_LOPROC: u32 = 0x70000000;
pub const TYPE_HIPROC: u32 = 0x7fffffff;

pub const FLAG_X: u32        =        0x1;
pub const FLAG_W: u32        =        0x2;
pub const FLAG_R: u32        =        0x4;
pub const FLAG_MASKOS: u32   = 0x0ff00000;
pub const FLAG_MASKPROC: u32 = 0xf0000000;

pub fn sanity_check<'a>(ph: ProgramHeader<'a>, elf_file: &ElfFile<'a>) -> Result<(), &'static str> {
    let header = elf_file.header;
    match ph {
        ProgramHeader::Ph32(ph) => {
            check!(mem::size_of_val(ph) == header.pt2.ph_entry_size() as usize,
                   "program header size mismatch");
            check!(((ph.offset + ph.file_size) as usize) < elf_file.input.len(),
                   "entry point out of range");
            check!(ph.get_type() != Type::ShLib, "Shouldn't use ShLib");
            if ph.align > 1 {
                check!(ph.virtual_addr % ph.align == ph.offset % ph.align,
                       "Invalid combination of virtual_addr, offset, and align");
            }
        }
        ProgramHeader::Ph64(ph) => {
            check!(mem::size_of_val(ph) == header.pt2.ph_entry_size() as usize,
                   "program header size mismatch");
            check!(((ph.offset + ph.file_size) as usize) < elf_file.input.len(),
                   "entry point out of range");
            check!(ph.get_type() != Type::ShLib, "Shouldn't use ShLib");
            if ph.align > 1 {
                println!("{} {} {}", ph.virtual_addr, ph.offset, ph.align);
                check!(ph.virtual_addr % ph.align == ph.offset % ph.align,
                       "Invalid combination of virtual_addr, offset, and align");
            }
        }
    }

    Ok(())
}
