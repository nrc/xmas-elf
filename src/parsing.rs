use std::mem;
use std::raw::Slice;

// These functions are all super-unsafe!

// fn parse_buf<'a, T>(input: &'a [u8], count: usize) -> &'a [T] {
//     unsafe {
//         assert!(input.len() == count * mem::size_of::<T>());
//         let mut data: &'a [T] = mem::transmute(input);
//         let data_slice: &mut Slice<T> = mem::transmute(&mut data);
//         data_slice.len = count;
//         data
//     }    
// }

pub fn parse_one<'a, T>(input: &'a [u8]) -> &'a T {
    unsafe {
        assert!(input.len() == mem::size_of::<T>());
        let data: &'a [T] = mem::transmute(input);
        let data_slice: &Slice<T> = mem::transmute(&data);
        mem::transmute(data_slice.data)
    }    
}

// The caller must ensure that input + offset points to the first byte of a
// null-terminated string and that the whole string has lifetime 'a.
pub fn parse_str<'a>(input: &'a u8, offset: usize) -> &'a str {
    unsafe {
        let input = input as *const u8;
        let input = input.offset(offset as isize);
        let mut cur = input;
        while *cur != 0 {
            cur = cur.offset(1);
        }
        let str_slice = Slice { data: input, len: cur as usize - input as usize };
        mem::transmute(str_slice)
    }
}
