use std::mem::transmute;
use std::slice::from_raw_parts;

pub fn convert_usize_to_u8(num: usize) -> Vec<u8> {
    Vec::from(unsafe { transmute::<u32, [u8; 4]>(num.to_be() as u32) })
}

pub fn convert_vec_to_u8<T>(vec: &Vec<T>) -> Vec<u8> {
    Vec::from(unsafe {
        from_raw_parts(
            vec.as_ptr() as *const u8,
            vec.len() * std::mem::size_of::<T>(),
        )
    })
}
