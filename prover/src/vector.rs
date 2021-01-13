pub struct _3DVector {
    data: Vec<u32>,
    row: u64,
    col: u64,
    area: u64,
    depth: u64,
}

impl _3DVector {
    pub fn new(depth: u64, row: u64, col: u64) -> Self {
        let area = row * col;
        _3DVector {
            data: vec![0; depth as usize * area as usize],
            row,
            col,
            area,
            depth,
        }
    }

    pub fn get_index(&self, d: u32, r: u32, c: u32) -> u64 {
        d as u64 * self.area + self.col * r as u64 + c as u64
    }

    pub fn get_size(&self) -> u64 {
        self.depth * self.area * 4
    }

    pub fn count_elements(&self) -> u64 {
        self.depth * self.area
    }
}
