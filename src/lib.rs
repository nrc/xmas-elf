#![feature(raw)]

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


pub mod header;
mod parsing;
pub mod sections;
pub mod program;
pub mod symbol_table;
pub mod dynamic;
pub mod hash;

use std::fs::File;
use std::io::Read;

use header::Header;
use sections::{SectionHeader, SectionIter};
use program::{ProgramHeader, ProgramIter};
use parsing::parse_str;
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

    pub fn section_header(&self, index: u16) -> SectionHeader<'a> {
        sections::parse_section_header(self.input, self.header, index)
    }

    pub fn section_iter<'b: 'a>(&'b self) -> SectionIter<'b, 'a> {
        SectionIter {
            file: &self,
            next_index: 0,
        }
    }

    pub fn program_header(&self, index: u16) -> ProgramHeader<'a> {
        program::parse_program_header(self.input, self.header, index)
    }    

    pub fn program_iter<'b: 'a>(&'b self) -> ProgramIter<'b, 'a> {
        ProgramIter {
            file: &self,
            next_index: 0,
        }
    }

    pub fn get_string(&self, index: u32) -> &'a str {
        parse_str(self.get_str_table(), index as usize)
    }

    // This is really, stupidly slow. Not sure how to fix that, perhaps keeping
    // a HashTable mapping names to section header indices?
    pub fn find_section_by_name(&'a self, name: &str) -> Option<SectionHeader<'a>> {
        for sect in self.section_iter() {
            if let Ok(sect_name) = sect.get_name(&self) {
                if sect_name == name {
                    return Some(sect);
                }
            }
        }

        None
    }

    fn get_str_table(&self) -> &'a u8 {
        // TODO cache this?
        let header = self.section_header(self.header.pt2.sh_str_index());
        &self.input[header.offset() as usize]
    }
}

// Note if running on a 32bit system, then reading Elf64 files probably will not
// work (maybe if the size of the file in bytes is < u32::Max).



// Helper function to open a file and read it into a buffer.
// Allocates the buffer.
pub fn open_file(name: &str) -> Vec<u8> {
    let mut f = File::open(name).unwrap();
    let mut buf = Vec::new();
    assert!(f.read_to_end(&mut buf).unwrap() > 0);
    buf
}

#[cfg(test)]
mod test {
    use super::*;
}
