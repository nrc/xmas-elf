#![feature(raw)]

mod header;
mod parsing;
mod sections;

use std::fs::File;
use std::io::Read;
use std::mem;

use header::{Header, parse_header};
use sections::parse_section_header;


pub type P32 = u32;
pub type P64 = u64;

// Note if running on a 32bit system, then reading Elf64 files probably will not
// work (maybe if the size of the file in bytes is < u32::Max).

// TODO move to header
fn sanity_check(header: &Header, input: &[u8]) -> Result<(), &'static str> {
    use header::{HeaderPt1, HeaderPt2, Class, MAGIC};

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

    check!(mem::size_of::<HeaderPt1>() == 16);
    check!(header.pt1.magic == MAGIC, "bad magic number");
    check!(mem::size_of::<HeaderPt1>() + header.pt2.size() == header.pt2.header_size() as usize,
           "header_size does not match size of header");
    match (&header.pt1.class, &header.pt2) {
        (&Class::None, _) => return Err("No class"),
        (&Class::ThirtyTwo, &HeaderPt2::Header32(_)) |
        (&Class::SixtyFour, &HeaderPt2::Header64(_)) => {}
        _ => return Err("Mismatch between specified and actual class"),
    }
    check!(!header.pt1.version.is_none(), "no version");
    check!(!header.pt1.data.is_none(), "no data format");

    check!(header.pt2.entry_point() < input.len() as u64, "entry point out of range");
    check!(header.pt2.ph_offset() + (header.pt2.ph_entry_size() as u64) * (header.pt2.ph_count() as u64)
           <= input.len() as u64, "program header table out of range");
    check!(header.pt2.sh_offset() + (header.pt2.sh_entry_size() as u64) * (header.pt2.sh_count() as u64)
           <= input.len() as u64, "section header table out of range");

    // TODO check that SectionHeader_ is the same size as sh_entry_size, depending on class

    Ok(())
}

fn main() {
    let filename = "foo.o";
    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();
    assert!(f.read_to_end(&mut buf).unwrap() > 0);
    let header = parse_header(&buf);
    println!("{}", header);
    sanity_check(&header, &buf).unwrap();
    let sect = parse_section_header(&buf, header, 10);
    println!("{}", sect);
    println!("{}", sect.name_as_str(&buf, header));
}


#[cfg(test)]
mod test {
    use super::*;
    use std::mem;

    #[test]
    fn test_empty() {
        let input: [u32; 1] = [0];
        let input: [u8; 4] = unsafe { mem::transmute(input) };
        assert!(parse(&input) == TempData(Vec::new()));
        assert!(parse_no_copy(&input) == DataNoCopy(&[]));
    }

    #[test]
    fn test_one() {
        let input: [u32; 2] = [1, 42];
        let input: [u8; 8] = unsafe { mem::transmute(input) };
        assert!(parse(&input) == TempData(vec![42]));
        assert!(parse_no_copy(&input) == DataNoCopy(&[42]));
    }
}


// #![feature(custom_derive, plugin)]
// #![plugin(serde_macros)]

// extern crate serde;
// #[macro_use]
// extern crate nom;



// impl Deserialize for Data {
//     fn deserialize<D>(deserializer: &mut D) -> Result<i32, D::Error>
//         where D: serde::Deserializer,
//     {
//         deserializer.visit(DataVisitor::new())
//     }
// }

// struct DataVisitor {
//     buf: Option<Vec<i32>>,
// }

// impl DataVisitor {
//     fn new() -> DataVisitor {
//         DataVisitor {
//             buf: None,
//         }
//     }
// }

// impl serde::de::Visitor for DataVisitor {
//     type Value = Data;
// }


// extern crate byteorder;

// use byteorder::{ReadBytesExt, LittleEndian};


// #[derive(PartialEq, Eq, Debug)]
// pub struct TempData(Vec<i32>);

// pub fn parse(input: &[u8]) -> TempData {
//     let mut cursor = Cursor::new(input);
//     let count = cursor.read_u32::<LittleEndian>().unwrap() as usize;
//     let mut result = TempData(Vec::with_capacity(count));
//     for _ in 0..count {
//         result.0.push(cursor.read_i32::<LittleEndian>().unwrap());
//     }
//     result
// }

// #[derive(PartialEq, Eq, Debug)]
// pub struct DataNoCopy<'a>(&'a [i32]);

// pub fn parse_no_copy<'a>(input: &'a [u8]) -> DataNoCopy<'a> {
//     let count = read_u32(input) as usize;
//     DataNoCopy(parse_buf(&input[4..], count))
// }

// fn read_u32(input: &[u8]) -> u32 {
//     unsafe {
//         assert!(input.len() >= 4);
//         *(mem::transmute::<_, &u32>(&input[0]))
//     }
// }
