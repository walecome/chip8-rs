pub struct Display {
    pub width: u32,
    pub height: u32,
    data: Vec<bool>,
}

impl Display {
    pub fn new() -> Display {
        let width: u32 = 64;
        let height: u32 = 32;
        Display {
            width,
            height,
            data: vec![false; (width * height) as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        return self.data[(y * self.width + x) as usize];
    }

    pub fn set(&mut self, x: u32, y: u32, value: bool) {
        self.data[(y * self.width + x) as usize] = value;
    }
}
