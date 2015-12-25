#![feature(raw)]

mod header;
mod parsing;
mod sections;
mod symbol_table;
mod dynamic;
mod hash;

use std::fs::File;
use std::io::Read;

use header::Header;
use sections::{SectionHeader, SectionIter, ShType};
use parsing::parse_str;
use symbol_table::Entry;

pub type P32 = u32;
pub type P64 = u64;

pub struct ElfFile<'a> {
    input: &'a [u8],
    header: Header<'a>,
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
        sections::parse_section_header(&self.input, self.header, index)
    }

    pub fn section_iter<'b: 'a>(&'b self) -> SectionIter<'b, 'a> {
        SectionIter {
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
        // TODO cache this
        let header = sections::parse_section_header(self.input,
                                                    self.header,
                                                    self.header.pt2.sh_str_index());
        &self.input[header.offset() as usize]
    }
}

// Note if running on a 32bit system, then reading Elf64 files probably will not
// work (maybe if the size of the file in bytes is < u32::Max).

// TODO make this whole thing more library-like
fn main() {
    let buf = open_file("foo.o");
    let elf_file = ElfFile::new(&buf);
    println!("{}", elf_file.header);
    header::sanity_check(&elf_file).unwrap();

    let mut sect_iter = elf_file.section_iter();
    // Skip the first (dummy) section
    sect_iter.next();
    for sect in sect_iter {
        println!("{}", sect.get_name(&elf_file).unwrap());
        println!("{:?}", sect.get_type());
        //println!("{}", sect);
        sections::sanity_check(sect, &elf_file).unwrap();

        if sect.get_type() == ShType::StrTab {
            println!("{:?}", sect.get_data(&elf_file).to_strings().unwrap());
        }

        if sect.get_type() == ShType::SymTab {
            if let sections::SectionData::SymbolTable64(data) = sect.get_data(&elf_file) {
                for datum in data {
                    println!("{}", datum.get_name(&elf_file));
                }
            } else {
                unreachable!();
            }
        }
    }

    let sect = elf_file.find_section_by_name(".rodata.const2794").unwrap();
    println!("{}", sect);
}

// Helper function to open a file and read it into a buffer.
// Allocates the buffer.
fn open_file(name: &str) -> Vec<u8> {
    let mut f = File::open(name).unwrap();
    let mut buf = Vec::new();
    assert!(f.read_to_end(&mut buf).unwrap() > 0);
    buf
}

#[cfg(test)]
mod test {
    use super::*;
}
