use sdl2::keyboard::Scancode;

#[derive(Debug, Clone)]
pub enum Keycode {
    Key0 = 0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
}

pub struct Keypad {
    down_key_map: Vec<bool>,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            down_key_map: vec![false; Keycode::KeyF as usize + 1],
        }
    }

    pub fn require_from(raw: u32) -> Keycode {
        match raw {
            x if x == Keycode::Key0 as u32 => Keycode::Key0,
            x if x == Keycode::Key1 as u32 => Keycode::Key1,
            x if x == Keycode::Key2 as u32 => Keycode::Key2,
            x if x == Keycode::Key3 as u32 => Keycode::Key3,
            x if x == Keycode::Key4 as u32 => Keycode::Key4,
            x if x == Keycode::Key5 as u32 => Keycode::Key5,
            x if x == Keycode::Key6 as u32 => Keycode::Key6,
            x if x == Keycode::Key7 as u32 => Keycode::Key7,
            x if x == Keycode::Key8 as u32 => Keycode::Key8,
            x if x == Keycode::Key9 as u32 => Keycode::Key9,
            x if x == Keycode::KeyA as u32 => Keycode::KeyA,
            x if x == Keycode::KeyB as u32 => Keycode::KeyB,
            x if x == Keycode::KeyC as u32 => Keycode::KeyC,
            x if x == Keycode::KeyD as u32 => Keycode::KeyD,
            x if x == Keycode::KeyE as u32 => Keycode::KeyE,
            x if x == Keycode::KeyF as u32 => Keycode::KeyF,
            _ => panic!("Invalid keycode: {}", raw),
        }
    }

    pub fn get_first_pressed_key(&self) -> Option<Keycode> {
        for (i, is_down) in (&self.down_key_map).into_iter().enumerate() {
            if *is_down {
                return Some(Keypad::require_from(i as u32));
            }
        }
        return None;
    }

    pub fn is_down(&self, keycode: Keycode) -> bool {
        return self.down_key_map[keycode as usize];
    }

    pub fn on_down(&mut self, scancode: Scancode) {
        match self.decode(scancode) {
            Some(keycode) => {
                self.down_key_map[keycode as usize] = true;
            }
            None => {}
        }
    }

    pub fn on_up(&mut self, scancode: Scancode) {
        match self.decode(scancode) {
            Some(keycode) => {
                self.down_key_map[keycode as usize] = false;
            }
            None => {}
        }
    }

    fn decode(&self, scancode: Scancode) -> Option<Keycode> {
        match scancode {
            Scancode::Num1 => Some(Keycode::Key1),
            Scancode::Num2 => Some(Keycode::Key2),
            Scancode::Num3 => Some(Keycode::Key3),
            Scancode::Num4 => Some(Keycode::KeyC),

            Scancode::Q => Some(Keycode::Key4),
            Scancode::W => Some(Keycode::Key5),
            Scancode::E => Some(Keycode::Key6),
            Scancode::R => Some(Keycode::KeyD),

            Scancode::A => Some(Keycode::Key7),
            Scancode::S => Some(Keycode::Key8),
            Scancode::D => Some(Keycode::Key9),
            Scancode::F => Some(Keycode::KeyE),

            Scancode::Z => Some(Keycode::KeyA),
            Scancode::X => Some(Keycode::Key0),
            Scancode::C => Some(Keycode::KeyB),
            Scancode::V => Some(Keycode::KeyF),

            _ => None,
        }
    }
}
