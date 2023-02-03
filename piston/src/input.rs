use piston_window::*;

pub fn button_piston_to_common(button: Button) -> Option<common::Button> {
	use common::Button as B;
	match button {
		Button::Keyboard(key) => match key {
			Key::Backspace => Some(B::KeyBackspace),
			Key::Tab => Some(B::KeyTab),
			Key::Return => Some(B::KeyReturn),
			Key::Escape => Some(B::KeyEscape),
			Key::Space => Some(B::KeySpace),
			Key::Exclaim => Some(B::KeyExclaim),
			Key::Quotedbl => Some(B::KeyDoubleQuote),
			Key::Hash => Some(B::KeyHash),
			Key::Dollar => Some(B::KeyDollar),
			Key::Percent => Some(B::KeyPercent),
			Key::Ampersand => Some(B::KeyAmpersand),
			Key::Quote => Some(B::KeyQuote),
			Key::LeftParen => Some(B::KeyLeftParen),
			Key::RightParen => Some(B::KeyRightParen),
			Key::Asterisk => Some(B::KeyAsterisk),
			Key::Plus => Some(B::KeyPlus),
			Key::Comma => Some(B::KeyComma),
			Key::Minus => Some(B::KeyMinus),
			Key::Period => Some(B::KeyPeriod),
			Key::Slash => Some(B::KeySlash),
			Key::D0 => Some(B::Key0),
			Key::D1 => Some(B::Key1),
			Key::D2 => Some(B::Key2),
			Key::D3 => Some(B::Key3),
			Key::D4 => Some(B::Key4),
			Key::D5 => Some(B::Key5),
			Key::D6 => Some(B::Key6),
			Key::D7 => Some(B::Key7),
			Key::D8 => Some(B::Key8),
			Key::D9 => Some(B::Key9),
			Key::Colon => Some(B::KeyColon),
			Key::Semicolon => Some(B::KeySemicolon),
			Key::Less => Some(B::KeyLess),
			Key::Equals => Some(B::KeyEquals),
			Key::Greater => Some(B::KeyGreater),
			Key::Question => Some(B::KeyQuestion),
			Key::At => Some(B::KeyAt),
			Key::LeftBracket => Some(B::KeyLeftBracket),
			Key::Backslash => Some(B::KeyBackslash),
			Key::RightBracket => Some(B::KeyRightBracket),
			Key::Caret => Some(B::KeyCaret),
			Key::Underscore => Some(B::KeyUnderscore),
			Key::Backquote => Some(B::KeyBackquote),
			Key::A => Some(B::KeyA),
			Key::B => Some(B::KeyB),
			Key::C => Some(B::KeyC),
			Key::D => Some(B::KeyD),
			Key::E => Some(B::KeyE),
			Key::F => Some(B::KeyF),
			Key::G => Some(B::KeyG),
			Key::H => Some(B::KeyH),
			Key::I => Some(B::KeyI),
			Key::J => Some(B::KeyJ),
			Key::K => Some(B::KeyK),
			Key::L => Some(B::KeyL),
			Key::M => Some(B::KeyM),
			Key::N => Some(B::KeyN),
			Key::O => Some(B::KeyO),
			Key::P => Some(B::KeyP),
			Key::Q => Some(B::KeyQ),
			Key::R => Some(B::KeyR),
			Key::S => Some(B::KeyS),
			Key::T => Some(B::KeyT),
			Key::U => Some(B::KeyU),
			Key::V => Some(B::KeyV),
			Key::W => Some(B::KeyW),
			Key::X => Some(B::KeyX),
			Key::Y => Some(B::KeyY),
			Key::Z => Some(B::KeyZ),
			Key::Delete => Some(B::KeyDelete),
			_ => None
		},
		Button::Mouse(mouse_buton) => match mouse_buton {
			MouseButton::Left => Some(B::MouseLeft),
			MouseButton::Right => Some(B::MouseRight),
			MouseButton::Middle => Some(B::MouseMiddle),
			_ => None
		},
		_ => None
	}
}