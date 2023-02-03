use common::*;

pub struct GameState {
	y: f64
}

pub fn requested_size() -> (u32, u32) {
	(320, 240)	
}

pub fn requested_tickrate() -> u32 {
	60
}

pub fn init(interface: &mut PlatformInterface) -> GameState {
	interface.set_palette_color(0, [0, 0, 0, 255]);
	interface.set_palette_color(1, [255, 0, 0, 255]);

	GameState { y: 0.0 }
}

pub fn tick(state: &mut GameState, interface: &mut PlatformInterface, delta: f64) {
	state.y += 5.0 * delta;
}

pub fn draw(state: &mut GameState, interface: &mut PlatformInterface, time: f64) {
	interface.set_background(0);
	interface.set_pixel(state.y, state.y, 1);
}