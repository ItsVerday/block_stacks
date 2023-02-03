use common::*;
use rand::Rng;

pub struct GameState {
}

pub fn requested_size() -> (u32, u32) {
	(320, 240)	
}

pub fn requested_tickrate() -> u32 {
	10
}

pub fn init(interface: &mut PlatformInterface) -> GameState {
	interface.set_palette_color(0, [0, 0, 0, 255]);
	interface.set_palette_color(1, [255, 0, 0, 255]);
	interface.set_palette_color(2, [255, 255, 0, 255]);
	interface.set_palette_color(3, [0, 255, 0, 255]);
	interface.set_palette_color(4, [0, 255, 255, 255]);
	interface.set_palette_color(5, [0, 0, 255, 255]);
	interface.set_palette_color(6, [255, 0, 255, 255]);
	interface.set_palette_color(7, [255, 255, 255, 255]);

	GameState {}
}

pub fn tick(state: &mut GameState, interface: &mut PlatformInterface, delta: f64) {
	let c = interface.rng.gen_range(1..=7);

	for _ in 0..10 {
		let x = interface.rng.gen_range(0..interface.width);
		let y = interface.rng.gen_range(0..interface.height);
		interface.set_pixel(x as f64, y as f64, c);
	}
}

pub fn draw(game_state: &mut GameState, interface: &mut PlatformInterface, time: f64) {
	// interface.set_background(0);
}