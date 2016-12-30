#![no_std]

// TODO move to a module
macro_rules! check {
    ($e:expr) => {
        if !$e {
            return Err("");
        }
    };
    ($e:expr, $msg: expr) => {
        if !$e {
            return Err($msg);
        }
    };
}

extern crate zero;

pub mod header;
pub mod sections;
pub mod program;
pub mod symbol_table;
pub mod dynamic;
pub mod hash;

use header::Header;
use sections::{SectionHeader, SectionIter};
use program::{ProgramHeader, ProgramIter};
use zero::read_str;
use symbol_table::Entry;

pub type P32 = u32;
pub type P64 = u64;

pub struct ElfFile<'a> {
    pub input: &'a [u8],
    pub header: Header<'a>,
}

impl<'a> ElfFile<'a> {
    pub fn new(input: &'a [u8]) -> ElfFile<'a> {
        let header = header::parse_header(&input);
        ElfFile {
            input: input,
            header: header,
        }
    }

    pub fn section_header(&self, index: u16) -> Result<SectionHeader<'a>, &'static str> {
        sections::parse_section_header(self.input, self.header, index)
    }

    pub fn section_iter<'b>(&'b self) -> SectionIter<'b, 'a> {
        SectionIter {
            file: &self,
            next_index: 0,
        }
    }

    pub fn program_header(&self, index: u16) -> Result<ProgramHeader<'a>, &'static str> {
        program::parse_program_header(self.input, self.header, index)
    }

    pub fn program_iter<'b>(&'b self) -> ProgramIter<'b, 'a> {
        ProgramIter {
            file: &self,
            next_index: 0,
        }
    }

    pub fn get_string(&self, index: u32) -> Result<&'a str, &'static str> {
        self.get_str_table().map(|str_table| read_str(&str_table[(index as usize)..]))
    }

    pub fn get_dyn_string(&self, index: u32) -> Result<&'a str, &'static str> {
        let header = self.find_section_by_name(".dynstr").unwrap();
        Ok(read_str(&header.raw_data(self)[(index as usize)..]))
    }

    // This is really, stupidly slow. Not sure how to fix that, perhaps keeping
    // a HashTable mapping names to section header indices?
    pub fn find_section_by_name(&self, name: &str) -> Option<SectionHeader<'a>> {
        for sect in self.section_iter() {
            if let Ok(sect_name) = sect.get_name(&self) {
                if sect_name == name {
                    return Some(sect);
                }
            }
        }

        None
    }

    fn get_str_table(&self) -> Result<&'a [u8], &'static str> {
        // TODO cache this?
        let header = self.section_header(try!(self.header.pt2).sh_str_index());
        header.map(|h| &self.input[(h.offset() as usize)..])
    }
}

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod test {
    use std::prelude::v1::*;

    use std::mem;

    use super::*;
    use header::{HeaderPt1, HeaderPt2_};

    fn mk_elf_header(class: u8) -> Vec<u8> {
        let header_size = mem::size_of::<HeaderPt1>() +
                          match class {
            1 => mem::size_of::<HeaderPt2_<P32>>(),
            2 => mem::size_of::<HeaderPt2_<P64>>(),
            _ => 0,
        };
        let mut header = vec![0x7f, 'E' as u8, 'L' as u8, 'F' as u8];
        let data = 1u8;
        let version = 1u8;
        header.extend_from_slice(&[class, data, version]);
        header.resize(header_size, 0);
        header
    }

    #[test]
    fn interpret_class() {
        assert!(ElfFile::new(&mk_elf_header(0)).header.pt2.is_err());
        assert!(ElfFile::new(&mk_elf_header(1)).header.pt2.is_ok());
        assert!(ElfFile::new(&mk_elf_header(2)).header.pt2.is_ok());
        assert!(ElfFile::new(&mk_elf_header(42u8)).header.pt2.is_err());
    }
}
