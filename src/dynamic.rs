use core::fmt;
use {P32, P64};
use zero::Pod;

pub struct Dynamic<P> {
    tag: Tag_<P>,
    un: P,
}

unsafe impl<P> Pod for Dynamic<P> {}

#[derive(Copy, Clone)]
pub struct Tag_<P>(P);

#[derive(Debug, PartialEq, Eq)]
pub enum Tag<P> {
    Null,
    Needed,
    PltRelSize,
    Pltgot,
    Hash,
    StrTab,
    SymTab,
    Rela,
    RelaSize,
    RelaEnt,
    StrSize,
    SymEnt,
    Init,
    Fini,
    SoName,
    RPath,
    Symbolic,
    Rel,
    RelSize,
    RelEnt,
    PltRel,
    Debug,
    TextRel,
    JmpRel,
    BindNow,
    InitArray,
    FiniArray,
    InitArraySize,
    FiniArraySize,
    RunPath,
    Flags,
    PreInitArray,
    PreInitArraySize,
    SymTabShIndex,
    OsSpecific(P),
    ProcessorSpecific(P),
}

macro_rules! impls {
    ($p: ident) => {
        impl Dynamic<$p> {
            pub fn get_tag(&self) -> Tag<$p> {
                self.tag.as_tag()
            } 

            pub fn get_val(&self) -> $p {
                match self.get_tag() {
                    Tag::Needed | Tag::PltRelSize | Tag::RelaSize | Tag::RelaEnt | Tag::StrSize |
                    Tag::SymEnt | Tag::SoName | Tag::RPath | Tag::RelSize | Tag::RelEnt | Tag::PltRel |
                    Tag::InitArraySize | Tag::FiniArraySize | Tag::RunPath | Tag::Flags |
                    Tag::PreInitArraySize | Tag::OsSpecific(_) | Tag::ProcessorSpecific(_) => self.un,
                    _ => panic!("val is not valid"),
                }
            }

            pub fn get_ptr(&self) -> $p {
                match self.get_tag() {
                   Tag::Pltgot | Tag::Hash | Tag::StrTab | Tag::SymTab | Tag::Rela | Tag::Init | Tag::Fini |
                   Tag::Rel | Tag::Debug | Tag::JmpRel | Tag::InitArray | Tag::FiniArray |
                   Tag::PreInitArray | Tag::SymTabShIndex  | Tag::OsSpecific(_) | Tag::ProcessorSpecific(_)
                   => self.un,
                    _ => panic!("ptr is not valid"),
                }
            }
        }

        impl Tag_<$p> {
            fn as_tag(self) -> Tag<$p> {
                match self.0 {
                    0 => Tag::Null,
                    1 => Tag::Needed,
                    2 => Tag::PltRelSize,
                    3 => Tag::Pltgot,
                    4 => Tag::Hash,
                    5 => Tag::StrTab,
                    6 => Tag::SymTab,
                    7 => Tag::Rela,
                    8 => Tag::RelaSize,
                    9 => Tag::RelaEnt,
                    10 => Tag::StrSize,
                    11 => Tag::SymEnt,
                    12 => Tag::Init,
                    13 => Tag::Fini,
                    14 => Tag::SoName,
                    15 => Tag::RPath,
                    16 => Tag::Symbolic,
                    17 => Tag::Rel,
                    18 => Tag::RelSize,
                    19 => Tag::RelEnt,
                    20 => Tag::PltRel,
                    21 => Tag::Debug,
                    22 => Tag::TextRel,
                    23 => Tag::JmpRel,
                    24 => Tag::BindNow,
                    25 => Tag::InitArray,
                    26 => Tag::FiniArray,
                    27 => Tag::InitArraySize,
                    28 => Tag::FiniArraySize,
                    29 => Tag::RunPath,
                    30 => Tag::Flags,
                    32 => Tag::PreInitArray,
                    33 => Tag::PreInitArraySize,
                    34 => Tag::SymTabShIndex,
                    t if t >= 0x6000000D && t <= 0x6fffffff => Tag::OsSpecific(t),
                    t if t >= 0x70000000 && t <= 0x7fffffff => Tag::ProcessorSpecific(t),
                    _ => panic!("Invalid value for tag"),
                }
            }
        }

        impl fmt::Debug for Tag_<$p> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.as_tag().fmt(f)
            }
        }
    }
}

impls!(P32);
impls!(P64);
