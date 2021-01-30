pub fn convert_usize_to_u8(num: usize) -> Vec<u8> {
    vec![
        (num >> 24) as u8,
        (num >> 16) as u8,
        (num >> 8) as u8,
        (num) as u8,
    ]
    // Vec::from(unsafe { transmute::<u32, [u8; 4]>(num.to_be() as u32) })
}

pub fn convert_u32_to_u8(vec: &[u32]) -> Vec<u8> {
    let mut res = vec![];
    for &val in vec.iter() {
        res.push((val >> 24) as u8);
        res.push((val >> 16) as u8);
        res.push((val >> 8) as u8);
        res.push(val as u8);
    }
    res
}
