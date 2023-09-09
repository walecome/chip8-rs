pub struct Keypad {}

pub enum Keycode {
    Code0,
    Code1,
    Code2,
    Code3,
    Code4,
    Code5,
    Code6,
    Code7,
    Code8,
    Code9,
    CodeA,
    CodeB,
    CodeC,
    CodeD,
    CodeE,
    CodeF,
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {}
    }

    fn decode(&self, raw: char) -> Option<Keycode> {
        None
    }
}
