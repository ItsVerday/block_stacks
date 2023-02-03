use std::collections::HashMap;
use rand::rngs::ThreadRng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type Color = [u8; 4];

pub struct PlatformInterface<'a> {
	pub width: usize,
	pub height: usize,

	pub pixel_buffer: Vec<u8>,
	pub palette: [Color; 256],

	pub mouse_pos: Option<(f64, f64)>,
	pub inputs: HashMap<Button, InputState>,

	pub rng: &'a mut ThreadRng
}

impl<'a> PlatformInterface<'a> {
	pub fn new(width: usize, height: usize, rng: &'a mut ThreadRng) -> PlatformInterface<'a> {
		let mut pixel_buffer = vec![];
		pixel_buffer.resize(width * height, 0);

		let mut inputs = HashMap::new();
		for button in Button::iter() {
			inputs.insert(button, InputState::Up);
		}

		PlatformInterface {
			width,
			height,
			pixel_buffer,
			palette: [[0; 4]; 256],
			mouse_pos: None,
			inputs,
			rng
		}
	}
	
	pub fn input(&self, button: Button) -> InputState {
		*self.inputs.get(&button).unwrap()
	}

	pub fn set_palette_color(&mut self, index: u8, color: Color) {
		self.palette[index as usize] = color;
	}

	pub fn set_pixel_exact(&mut self, x: isize, y: isize, color: u8) {
		if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
			self.pixel_buffer[(x + y * self.width as isize) as usize] = color;
		}
	}
	
	pub fn set_pixel(&mut self, x: f64, y: f64, color: u8) {
		self.set_pixel_exact(x.floor() as isize, y.floor() as isize, color);
	}

	pub fn set_background(&mut self, color: u8) {
		for x in 0..self.width {
			for y in 0..self.height {
				self.set_pixel_exact(x as isize, y as isize, color);
			}
		}
	}
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, EnumIter)]
pub enum InputState {
	Pressed,
	Down,
	Released,
	Up
}

impl InputState {
	pub fn is_down(&self) -> bool {
		match self {
			Self::Pressed | Self::Down => true,
			Self::Released | Self::Up => false
		}
	}

	pub fn is_first(&self) -> bool {
		match self {
			Self::Pressed | Self::Released => true,
			Self::Down | Self::Up => false
		}
	}

	pub fn advance(&self) -> Self {
		match self {
			Self::Pressed | Self::Down => Self::Down,
			Self::Released | Self::Up => Self::Up
		}
	}
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, EnumIter)]
pub enum Button {
	MouseLeft,
	MouseRight,
	MouseMiddle,
	KeyBackspace,
	KeyTab,
	KeyReturn,
	KeyEscape,
	KeySpace,
	KeyExclaim,
	KeyDoubleQuote,
	KeyHash,
	KeyDollar,
	KeyPercent,
	KeyAmpersand,
	KeyQuote,
	KeyLeftParen,
	KeyRightParen,
	KeyAsterisk,
	KeyPlus,
    KeyComma,
    KeyMinus,
    KeyPeriod,
    KeySlash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyColon,
    KeySemicolon,
    KeyLess,
    KeyEquals,
    KeyGreater,
    KeyQuestion,
    KeyAt,
    KeyLeftBracket,
    KeyBackslash,
    KeyRightBracket,
    KeyCaret,
    KeyUnderscore,
    KeyBackquote,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    KeyDelete
}