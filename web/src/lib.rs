use common::PlatformInterface;
use game::GameState;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
	#[wasm_bindgen(js_namespace = console, js_name = log)]
	fn log(s: &str);
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
}

#[wasm_bindgen]
pub fn tick_game(delta: f64) {
	unsafe {
		let game_state = GAME_STATE.as_mut().unwrap();
		let interface = INTERFACE.as_mut().unwrap();

		game::tick(game_state, interface, delta);
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