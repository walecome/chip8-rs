pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SubroutineReturn,
    SubroutineCall(u16),
    SetVX { register: u8, value: u8 },
    AddVX { register: u8, value: u8 },
    SetI(u16),
    DisplayDraw { x: u8, y: u8, n: u8 },
}
