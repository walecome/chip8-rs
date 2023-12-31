#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump(u16),
    SubroutineReturn,
    SubroutineCall(u16),
    SetVX {
        register: u8,
        value: u8,
    },
    AddVX {
        register: u8,
        value: u8,
    },
    SetI(u16),
    DisplayDraw {
        register_x: u8,
        register_y: u8,
        n: u8,
    },
    NoopImmediateEq {
        register: u8,
        value: u8,
    },
    NoopImmediateNotEq {
        register: u8,
        value: u8,
    },
    NoopRegisterEq {
        register_x: u8,
        register_y: u8,
    },
    NoopRegisterNotEq {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticSet {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticOr {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticAnd {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticXor {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticAdd {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticSubtractXY {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticSubtractYX {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticShiftLeft {
        register_x: u8,
        register_y: u8,
    },
    ArithmeticShiftRight {
        register_x: u8,
        register_y: u8,
    },
    Store(u8),
    Load(u8),
    BcdConversion(u8),
    FontCharacter(u8),
    AddToIndex(u8),
    SetVXFromDelayTimer(u8),
    SetDelayTimerFromVX(u8),
    SetSoundTimerFromVX(u8),
    NoopVXDown(u8),
    NoopVXNotDown(u8),
    JumpWithOffset {
        register_x: u8,
        address: u16,
    },
    GetKey(u8),
    Random {
        register_x: u8,
        mask: u8,
    }
}
