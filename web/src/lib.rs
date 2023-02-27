mod input;

use std::{collections::HashMap, ffi::OsStr};

use common::{PlatformInterface, Button, InputState};
use game::GameState;
use wasm_bindgen::prelude::*;
use include_dir::{include_dir, Dir};

static ASSETS_DIR: Dir = include_dir!("$ASSETS_DIR");

#[wasm_bindgen]
extern {
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Size {
	pub width: u32,
	pub height: u32
}

pub struct StaticData<'a> {
	pub interface: PlatformInterface<'a>,
	pub game_state: GameState,
	pub sounds: HashMap<&'a str, Vec<u8>>,
	pub sounds_to_play: Option<Vec<&'static str>>
}

static mut DATA: Option<StaticData<'static>> = None;

#[wasm_bindgen]
pub fn requested_size() -> Size {
	let (width, height) = game::requested_size();
	Size {
		width,
		height
	}
}

#[wasm_bindgen]
pub fn requested_tickrate() -> u32 {
	game::requested_tickrate()
}

#[wasm_bindgen]
pub fn init_game(width: u32, height: u32) {
	let mut interface = PlatformInterface::new(width as usize, height as usize, rand::thread_rng());
	let game_state = game::init(&mut interface);

	let mut sounds = HashMap::new();

	for file in ASSETS_DIR.get_dir("audio").unwrap().files() {
        if file.path().extension() != Some(OsStr::new("ogg")) {
            continue;
        }

        sounds.insert(file.path().file_stem().unwrap().to_str().unwrap(), file.contents().to_vec());
    }

	unsafe {
		DATA = Some(StaticData {
			interface,
			game_state,
			sounds,
			sounds_to_play: None
		});
	}
}

#[wasm_bindgen]
pub fn tick_game(delta: f64) {
	unsafe {
		let StaticData { interface, game_state, .. } = DATA.as_mut().unwrap();

		game::tick(game_state, interface, delta);

		let mut new_inputs = HashMap::new();

        for (button, state) in interface.inputs.iter() {
            new_inputs.insert(*button, state.advance());
        }

        interface.inputs = new_inputs;
	}
}

#[wasm_bindgen]
pub fn draw_game(time: f64) -> js_sys::Uint8Array {
	unsafe {
		let StaticData { interface, game_state, .. } = DATA.as_mut().unwrap();

		game::draw(game_state, interface, time);

		let mut draw_data = vec![];
		for palette_index in interface.pixel_buffer.iter() {
			let color = interface.palette[*palette_index as usize];
			draw_data.push(color[0]);
			draw_data.push(color[1]);
			draw_data.push(color[2]);
			draw_data.push(color[3]);
		}

		js_sys::Uint8Array::from(&draw_data[..])
	}
}

#[wasm_bindgen]
pub fn flush_sounds() -> usize {
	unsafe {
		let mut static_data = DATA.as_mut().unwrap();
		let sounds = static_data.interface.flush_play_sounds();
		let len = sounds.len();
		static_data.sounds_to_play = Some(sounds);

		len
	}
}

#[wasm_bindgen]
pub fn pop_sound_data() -> js_sys::Uint8Array {
	unsafe {
		let mut static_data = DATA.as_mut().unwrap();
		let sound_data = static_data.sounds_to_play.as_mut().unwrap().pop();
		
		js_sys::Uint8Array::from(&static_data.sounds.get(sound_data.unwrap()).unwrap()[..])
	}
}

#[wasm_bindgen]
pub fn handle_mouse_input(button: u8, pressed: bool) {
	unsafe {
		let StaticData { interface, .. } = DATA.as_mut().unwrap();
		let button = match button {
			0 => Button::MouseLeft,
			1 => Button::MouseMiddle,
			2 => Button::MouseRight,
			_ => return
		};

		let state = if pressed {InputState::Pressed} else {InputState::Released};

		interface.inputs.insert(button, state);
	}
}

#[wasm_bindgen]
pub fn handle_mouse_move(x: f64, y: f64) {
	unsafe {
		let StaticData { interface, .. } = DATA.as_mut().unwrap();
		interface.mouse_pos = Some((x, y));
	}
}

#[wasm_bindgen]
pub fn handle_key_input(key: &str, pressed: bool) {
	unsafe {
		let StaticData { interface, .. } = DATA.as_mut().unwrap();
		let button = input::key_name_to_common(key);

		let state = if pressed {InputState::Pressed} else {InputState::Released};

		if let Some(button) = button {
			interface.inputs.insert(button, state);
		}
	}
}