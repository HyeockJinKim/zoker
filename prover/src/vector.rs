#[derive(Debug, PartialEq)]
pub struct _3DVector {
    pub data: Vec<u32>,
    row: usize,
    col: usize,
    area: usize,
    depth: usize,
}

impl _3DVector {
    pub fn new(depth: usize, row: usize, col: usize) -> Self {
        let area: usize = row * col;
        _3DVector {
            data: vec![0; depth as usize * area as usize],
            row,
            col,
            area,
            depth,
        }
    }

    pub fn get_index(&self, d: usize, r: usize, c: usize) -> usize {
        d * self.area + self.col * r + c
    }
}
