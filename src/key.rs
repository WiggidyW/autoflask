use std::convert::TryFrom;
use rdev::{EventType, Key, Button};
use derive_more::Deref;

pub struct KeySequence(Vec<EventType>);

#[derive(Deref)]
pub struct KeyResponder(EventType);

impl super::Key for KeySequence {
    fn send(&self) {
        for event in self.0.iter() {
            rdev::simulate(event).unwrap();
        }
    }
}

impl TryFrom<String> for KeySequence {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let vec = match value.as_ref() {
            "1" => vec![EventType::KeyPress(Key::Num1), EventType::KeyRelease(Key::Num1)],
            "2" => vec![EventType::KeyPress(Key::Num2), EventType::KeyRelease(Key::Num2)],
            "3" => vec![EventType::KeyPress(Key::Num3), EventType::KeyRelease(Key::Num3)],
            "4" => vec![EventType::KeyPress(Key::Num4), EventType::KeyRelease(Key::Num4)],
            "5" => vec![EventType::KeyPress(Key::Num5), EventType::KeyRelease(Key::Num5)],
            "6" => vec![EventType::KeyPress(Key::Num6), EventType::KeyRelease(Key::Num6)],
            "7" => vec![EventType::KeyPress(Key::Num7), EventType::KeyRelease(Key::Num7)],
            "8" => vec![EventType::KeyPress(Key::Num8), EventType::KeyRelease(Key::Num8)],
            "9" => vec![EventType::KeyPress(Key::Num9), EventType::KeyRelease(Key::Num9)],
            "0" => vec![EventType::KeyPress(Key::Num0), EventType::KeyRelease(Key::Num0)],
            "MButton1" => vec![EventType::ButtonPress(Button::Left), EventType::ButtonRelease(Button::Left)],
            "MButton2" => vec![EventType::ButtonPress(Button::Right), EventType::ButtonRelease(Button::Right)],
            "MButton3" => vec![EventType::ButtonPress(Button::Middle), EventType::ButtonRelease(Button::Middle)],
            "MButton4" => vec![EventType::ButtonPress(Button::Unknown(1)), EventType::ButtonRelease(Button::Unknown(1))],
            "MButton5" => vec![EventType::ButtonPress(Button::Unknown(2)), EventType::ButtonRelease(Button::Unknown(2))],
            "q" => vec![EventType::KeyPress(Key::KeyQ), EventType::KeyRelease(Key::KeyQ)],
            "w" => vec![EventType::KeyPress(Key::KeyW), EventType::KeyRelease(Key::KeyW)],
            "e" => vec![EventType::KeyPress(Key::KeyE), EventType::KeyRelease(Key::KeyE)],
            "r" => vec![EventType::KeyPress(Key::KeyR), EventType::KeyRelease(Key::KeyR)],
            "t" => vec![EventType::KeyPress(Key::KeyT), EventType::KeyRelease(Key::KeyT)],
            "y" => vec![EventType::KeyPress(Key::KeyY), EventType::KeyRelease(Key::KeyY)],
            "u" => vec![EventType::KeyPress(Key::KeyU), EventType::KeyRelease(Key::KeyU)],
            "i" => vec![EventType::KeyPress(Key::KeyI), EventType::KeyRelease(Key::KeyI)],
            "o" => vec![EventType::KeyPress(Key::KeyO), EventType::KeyRelease(Key::KeyO)],
            "p" => vec![EventType::KeyPress(Key::KeyP), EventType::KeyRelease(Key::KeyP)],
            "a" => vec![EventType::KeyPress(Key::KeyA), EventType::KeyRelease(Key::KeyA)],
            "s" => vec![EventType::KeyPress(Key::KeyS), EventType::KeyRelease(Key::KeyS)],
            "d" => vec![EventType::KeyPress(Key::KeyD), EventType::KeyRelease(Key::KeyD)],
            "f" => vec![EventType::KeyPress(Key::KeyF), EventType::KeyRelease(Key::KeyF)],
            "g" => vec![EventType::KeyPress(Key::KeyG), EventType::KeyRelease(Key::KeyG)],
            "h" => vec![EventType::KeyPress(Key::KeyH), EventType::KeyRelease(Key::KeyH)],
            "j" => vec![EventType::KeyPress(Key::KeyJ), EventType::KeyRelease(Key::KeyJ)],
            "k" => vec![EventType::KeyPress(Key::KeyK), EventType::KeyRelease(Key::KeyK)],
            "l" => vec![EventType::KeyPress(Key::KeyL), EventType::KeyRelease(Key::KeyL)],
            "z" => vec![EventType::KeyPress(Key::KeyZ), EventType::KeyRelease(Key::KeyZ)],
            "x" => vec![EventType::KeyPress(Key::KeyX), EventType::KeyRelease(Key::KeyX)],
            "c" => vec![EventType::KeyPress(Key::KeyC), EventType::KeyRelease(Key::KeyC)],
            "v" => vec![EventType::KeyPress(Key::KeyV), EventType::KeyRelease(Key::KeyV)],
            "b" => vec![EventType::KeyPress(Key::KeyB), EventType::KeyRelease(Key::KeyB)],
            "n" => vec![EventType::KeyPress(Key::KeyN), EventType::KeyRelease(Key::KeyN)],
            "m" => vec![EventType::KeyPress(Key::KeyM), EventType::KeyRelease(Key::KeyM)],
            _ => return Err(format!("Unsupported Key: {}", &value)),
        };
        Ok(Self(vec))
    }
}

impl From<KeySequence> for KeyResponder {
    fn from(value: KeySequence) -> Self {
        Self(value.0[1])
    }
}