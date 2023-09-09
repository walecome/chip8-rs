#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SubroutineReturn,
    SubroutineCall(u16),
    SetVX { register: u8, value: u8 },
    AddVX { register: u8, value: u8 },
    SetI(u16),
    DisplayDraw { register_x: u8, register_y: u8, n: u8 },
}
