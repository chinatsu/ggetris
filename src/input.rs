use ggez::{GameResult, Context, filesystem};
use ggez::event::KeyCode;
use serde::{Deserialize};
use std::io::Read;

#[derive(Deserialize)]
struct TomlConfig {
    input: InputTomlConfig,
    game: GameTomlConfig
}

#[derive(Deserialize)]
struct InputTomlConfig {
    left: String,
    right: String,
    down: String,
    harddrop: String,
    rotate_cw: String,
    rotate_ccw: String,
    flip: String
}

#[derive(Deserialize)]
pub struct GameTomlConfig {
    pub das: u64
}

pub struct Config {
    pub input: InputConfig,
    pub game: GameTomlConfig
}

pub struct InputConfig {
    pub left: KeyCode,
    pub right: KeyCode,
    pub down: KeyCode,
    pub harddrop: KeyCode,
    pub rotate_cw: KeyCode,
    pub rotate_ccw: KeyCode,
    pub flip: KeyCode
}

impl Config {
    pub fn new(ctx: &mut Context) -> GameResult<Config> {
        let mut file = filesystem::open(ctx, "/config.toml")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let config: TomlConfig = toml::from_slice(&buffer).unwrap();

        Ok(Config {
            input: InputConfig {
                left: get_keycode(config.input.left),
                right: get_keycode(config.input.right),
                down: get_keycode(config.input.down),
                harddrop: get_keycode(config.input.harddrop),
                rotate_cw: get_keycode(config.input.rotate_cw),
                rotate_ccw: get_keycode(config.input.rotate_ccw),
                flip: get_keycode(config.input.flip),
            },
            game: config.game
        })
    }
}

fn get_keycode(s: String) -> KeyCode {
    match s.to_lowercase().as_str() {
        "1" => KeyCode::Key1,
        "2" => KeyCode::Key2,
        "3" => KeyCode::Key3,
        "4" => KeyCode::Key4,
        "5" => KeyCode::Key5,
        "6" => KeyCode::Key6,
        "7" => KeyCode::Key7,
        "8" => KeyCode::Key8,
        "9" => KeyCode::Key9,
        "0" => KeyCode::Key0,
        "escape" => KeyCode::Escape,
        "f1" => KeyCode::F1,
        "f2" => KeyCode::F2,
        "f3" => KeyCode::F3,
        "f4" => KeyCode::F4,
        "f5" => KeyCode::F5,
        "f6" => KeyCode::F6,
        "f7" => KeyCode::F7,
        "f8" => KeyCode::F8,
        "f9" => KeyCode::F9,
        "f10" => KeyCode::F10,
        "f11" => KeyCode::F11,
        "f12" => KeyCode::F12,
        "f13" => KeyCode::F13,
        "f14" => KeyCode::F14,
        "f15" => KeyCode::F15,
        "f16" => KeyCode::F16,
        "f17" => KeyCode::F17,
        "f18" => KeyCode::F18,
        "f19" => KeyCode::F19,
        "f20" => KeyCode::F20,
        "f21" => KeyCode::F21,
        "f22" => KeyCode::F22,
        "f23" => KeyCode::F23,
        "f24" => KeyCode::F24,
        "snapshot" => KeyCode::Snapshot,
        "scroll" => KeyCode::Scroll,
        "pause" => KeyCode::Pause,
        "insert" => KeyCode::Insert,
        "home" => KeyCode::Home,
        "delete" => KeyCode::Delete,
        "end" => KeyCode::End,
        "pagedown" => KeyCode::PageDown,
        "pageup" => KeyCode::PageUp,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "back" => KeyCode::Back,
        "return" => KeyCode::Return,
        "space" => KeyCode::Space,
        "compose" => KeyCode::Compose,
        "caret" => KeyCode::Caret,
        "numlock" => KeyCode::Numlock,
        "numpad0" => KeyCode::Numpad0,
        "numpad1" => KeyCode::Numpad1,
        "numpad2" => KeyCode::Numpad2,
        "numpad3" => KeyCode::Numpad3,
        "numpad4" => KeyCode::Numpad4,
        "numpad5" => KeyCode::Numpad5,
        "numpad6" => KeyCode::Numpad6,
        "numpad7" => KeyCode::Numpad7,
        "numpad8" => KeyCode::Numpad8,
        "numpad9" => KeyCode::Numpad9,
        "abntc1" => KeyCode::AbntC1,
        "abntc2" => KeyCode::AbntC2,
        "add" => KeyCode::Add,
        "apostrophe" => KeyCode::Apostrophe,
        "apps" => KeyCode::Apps,
        "at" => KeyCode::At,
        "ax" => KeyCode::Ax,
        "backslash" => KeyCode::Backslash,
        "calculator" => KeyCode::Calculator,
        "capital" => KeyCode::Capital,
        "colon" => KeyCode::Colon,
        "comma" => KeyCode::Comma,
        "convert" => KeyCode::Convert,
        "decimal" => KeyCode::Decimal,
        "divide" => KeyCode::Divide,
        "equals" => KeyCode::Equals,
        "grave" => KeyCode::Grave,
        "kana" => KeyCode::Kana,
        "kanji" => KeyCode::Kanji,
        "lalt" => KeyCode::LAlt,
        "lbracket" => KeyCode::LBracket,
        "lcontrol" => KeyCode::LControl,
        "lshift" => KeyCode::LShift,
        "lwin" => KeyCode::LWin,
        "mail" => KeyCode::Mail,
        "mediaselect" => KeyCode::MediaSelect,
        "mediastop" => KeyCode::MediaStop,
        "minus" => KeyCode::Minus,
        "multiply" => KeyCode::Multiply,
        "mute" => KeyCode::Mute,
        "mycomputer" => KeyCode::MyComputer,
        "navigateforward" => KeyCode::NavigateForward,
        "navigatebackward" => KeyCode::NavigateBackward,
        "nexttrack" => KeyCode::NextTrack,
        "noconvert" => KeyCode::NoConvert,
        "numpadcomma" => KeyCode::NumpadComma,
        "numpadenter" => KeyCode::NumpadEnter,
        "numpadequals" => KeyCode::NumpadEquals,
        "oem102" => KeyCode::OEM102,
        "period" => KeyCode::Period,
        "playpause" => KeyCode::PlayPause,
        "power" => KeyCode::Power,
        "prevtrack" => KeyCode::PrevTrack,
        "ralt" => KeyCode::RAlt,
        "rbracket" => KeyCode::RBracket,
        "rcontrol" => KeyCode::RControl,
        "rshift" => KeyCode::RShift,
        "rwin" => KeyCode::RWin,
        "semicolon" => KeyCode::Semicolon,
        "slash" => KeyCode::Slash,
        "sleep" => KeyCode::Sleep,
        "stop" => KeyCode::Stop,
        "subtract" => KeyCode::Subtract,
        "sysrq" => KeyCode::Sysrq,
        "tab" => KeyCode::Tab,
        "underline" => KeyCode::Underline,
        "unlabeled" => KeyCode::Unlabeled,
        "volumedown" => KeyCode::VolumeDown,
        "volumeup" => KeyCode::VolumeUp,
        "wake" => KeyCode::Wake,
        "webback" => KeyCode::WebBack,
        "webfavorites" => KeyCode::WebFavorites,
        "webforward" => KeyCode::WebForward,
        "webhome" => KeyCode::WebHome,
        "webrefresh" => KeyCode::WebRefresh,
        "websearch" => KeyCode::WebSearch,
        "webstop" => KeyCode::WebStop,
        "yen" => KeyCode::Yen,
        "copy" => KeyCode::Copy,
        "paste" => KeyCode::Paste,
        "cut" => KeyCode::Cut,
        "a" => KeyCode::A,
        "b" => KeyCode::B,
        "c" => KeyCode::C,
        "d" => KeyCode::D,
        "e" => KeyCode::E,
        "f" => KeyCode::F,
        "g" => KeyCode::G,
        "h" => KeyCode::H,
        "i" => KeyCode::I,
        "j" => KeyCode::J,
        "k" => KeyCode::K,
        "l" => KeyCode::L,
        "m" => KeyCode::M,
        "n" => KeyCode::N,
        "o" => KeyCode::O,
        "p" => KeyCode::P,
        "q" => KeyCode::Q,
        "r" => KeyCode::R,
        "s" => KeyCode::S,
        "t" => KeyCode::T,
        "u" => KeyCode::U,
        "v" => KeyCode::V,
        "w" => KeyCode::W,
        "x" => KeyCode::X,
        "y" => KeyCode::Y,
        "z" => KeyCode::Z,
        _ => KeyCode::WebStop,
    }
}
