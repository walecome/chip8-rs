pub struct Display {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Display {
    pub fn new() -> Display {
        let width: usize = 64;
        let height: usize = 32;
        Display {
            width,
            height,
            data: vec![false; width * height],
        }
    }
}
