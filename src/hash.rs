use core::mem;

use symbol_table::Entry;
use zero::{read, Pod};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct HashTableInner {
    bucket_count: u32,
    chain_count: u32,
    first_bucket: u32,
}

const OFFSET_OF_FIRST_BUCKET: usize = mem::offset_of!(HashTableInner, first_bucket);

#[derive(Clone, Copy, Debug)]
pub struct HashTable<'a> {
    inner: &'a HashTableInner,
    bounds: usize, // In number of u32s
}

unsafe impl Pod for HashTableInner {}

pub fn hash(input: &str) -> u32 {
    let mut result = 0;
    for i in input.bytes() {
        result = (result << 4) + i as u32;
        let g = result & 0xf0000000;
        if g != 0 {
            result ^= g >> 24;
        }
        result &= !g
    }
    result
}

impl<'a> HashTable<'a> {
    pub(crate) fn read(data: &'a [u8]) -> HashTable<'a> {
        HashTable {
            inner: read(&data[0..12]),
            bounds: (data.len() - OFFSET_OF_FIRST_BUCKET) / mem::size_of::<u32>(),
        }
    }

    pub fn get_bucket(&self, index: u32) -> u32 {
        assert!(index < self.inner.bucket_count);
        assert!((index as usize) < self.bounds);
        unsafe {
            let ptr = (&self.inner.first_bucket as *const u32).offset(index as isize);
            *ptr
        }
    }

    pub fn get_chain(&self, index: u32) -> u32 {
        assert!(index < self.inner.chain_count);
        let index = self.inner.bucket_count + index;
        assert!((index as usize) < self.bounds);
        unsafe {
            let ptr = (&self.inner.first_bucket as *const u32).offset(index as isize);
            *ptr
        }
    }

    pub fn lookup<F>(&'a self, _name: &str, _f: F) -> &'a dyn Entry
        where F: Fn(&dyn Entry) -> bool
    {
        // TODO
        unimplemented!();
    }
}
