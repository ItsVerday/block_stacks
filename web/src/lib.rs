mod input;

use std::collections::HashMap;

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


static mut GAME_STATE: Option<GameState> = None;
static mut INTERFACE: Option<PlatformInterface> = None;

#[wasm_bindgen]
pub fn init_game(width: u32, height: u32) {
	let mut interface = PlatformInterface::new(width as usize, height as usize, rand::thread_rng());
	let game_state = game::init(&mut interface);

	unsafe {
		INTERFACE = Some(interface);
		GAME_STATE = Some(game_state);
	}

	console_log!("{:?}", ASSETS_DIR);
}

#[wasm_bindgen]
pub fn tick_game(delta: f64) {
	unsafe {
		let game_state = GAME_STATE.as_mut().unwrap();
		let interface = INTERFACE.as_mut().unwrap();

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
		let game_state = GAME_STATE.as_mut().unwrap();
		let interface = INTERFACE.as_mut().unwrap();

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
pub fn handle_mouse_input(button: u8, pressed: bool) {
	unsafe {
		let interface = INTERFACE.as_mut().unwrap();
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
		let interface = INTERFACE.as_mut().unwrap();
		interface.mouse_pos = Some((x, y));
	}
}

#[wasm_bindgen]
pub fn handle_key_input(key: &str, pressed: bool) {
	unsafe {
		let interface = INTERFACE.as_mut().unwrap();
		let button = input::key_name_to_common(key);

		let state = if pressed {InputState::Pressed} else {InputState::Released};

		if let Some(button) = button {
			interface.inputs.insert(button, state);
		}
	}
}