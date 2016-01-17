extern crate xmas_elf;

use xmas_elf::{open_file, ElfFile, header, program};
use xmas_elf::sections::{self, ShType};

// TODO make this whole thing more library-like
fn main() {
    let buf = open_file("foo");
    let elf_file = ElfFile::new(&buf);
    println!("{}", elf_file.header);
    header::sanity_check(&elf_file).unwrap();

    let mut sect_iter = elf_file.section_iter();
    // Skip the first (dummy) section
    sect_iter.next();
    println!("sections");
    for sect in sect_iter {
        println!("{}", sect.get_name(&elf_file).unwrap());
        println!("{:?}", sect.get_type());
        //println!("{}", sect);
        sections::sanity_check(sect, &elf_file).unwrap();

        if sect.get_type() == ShType::StrTab {
            //println!("{:?}", sect.get_data(&elf_file).to_strings().unwrap());
        }

        if sect.get_type() == ShType::SymTab {
            if let sections::SectionData::SymbolTable64(data) = sect.get_data(&elf_file) {
                for datum in data {
                    //println!("{}", datum.get_name(&elf_file));
                }
            } else {
                unreachable!();
            }
        }
    }
    let ph_iter = elf_file.program_iter();
    println!("\nprogram headers");
    for sect in ph_iter {
        println!("{:?}", sect.get_type());
        program::sanity_check(sect, &elf_file).unwrap();
    }

    let sect = elf_file.program_header(5);
    println!("{}", sect);
    let data = sect.get_data(&elf_file);
    if let program::SegmentData::Note64(header, ptr) = data {
        println!("{}: {:?}", header.name(ptr), header.desc(ptr));
    }

    //let sect = elf_file.find_section_by_name(".rodata.const2794").unwrap();
    //println!("{}", sect);
}