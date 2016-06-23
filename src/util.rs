use core::mem;
use core::slice;

use header::Data;
use byteorder;
use byteorder::ByteOrder;

fn to_byte_slice<T: Copy>(reference: &T) -> &[u8] {
    unsafe {
        slice::from_raw_parts(reference as *const T as *const u8, mem::size_of::<T>())
    }
}

pub fn convert_endianess_u16(byte_ordering: Data, value: &mut u16) {
    match byte_ordering {
        Data::None => {},
        Data::LittleEndian => {
            *value = byteorder::LittleEndian::read_u16(to_byte_slice(value));
        },
        Data::BigEndian => {
            *value = byteorder::BigEndian::read_u16(to_byte_slice(value));
        }
    }
}


pub fn convert_endianess_u32(byte_ordering: Data, value: &mut u32) {
    match byte_ordering {
        Data::None => {},
        Data::LittleEndian => {
            *value = byteorder::LittleEndian::read_u32(to_byte_slice(value));
        },
        Data::BigEndian => {
            *value = byteorder::BigEndian::read_u32(to_byte_slice(value));
        }
    }
}

pub fn convert_endianess_u64(byte_ordering: Data, value: &mut u64) {
    match byte_ordering {
        Data::None => {},
        Data::LittleEndian => {
            *value = byteorder::LittleEndian::read_u64(to_byte_slice(value));
        },
        Data::BigEndian => {
            *value = byteorder::BigEndian::read_u64(to_byte_slice(value));
        }
    }
}

pub trait ResultExt<T, E: Clone> {
    fn ok_as_ref(&self) -> Result<&T, E>;
}

impl<T, E: Clone> ResultExt<T, E> for Result<T, E> {
    fn ok_as_ref(&self) -> Result<&T, E> {
        match *self {
            Ok(ref value) => Ok(value),
            Err(ref error) => Err(error.clone())
        }
    }
}
