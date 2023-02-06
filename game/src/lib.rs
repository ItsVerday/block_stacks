pub mod palette;
pub mod blocks {
	pub mod field;
	pub mod column;
	pub mod block;
	pub mod block_type;
}

use common::*;
use blocks::field::Field;

pub struct GameState {
	pub field: Field
}

pub fn requested_size() -> (u32, u32) {
    (360, 240)
}

pub fn requested_tickrate() -> u32 {
    60
}

const FIELD_WIDTH: u32 = 10;
const FIELD_HEIGHT: u32 = 20;
const BLOCK_SCALE: f64 = 10.0;

pub fn init(interface: &mut PlatformInterface) -> GameState {
	palette::load_palette(interface);

    GameState {
		field: Field::new(interface, FIELD_WIDTH, FIELD_HEIGHT)
	}
}

pub fn tick(state: &mut GameState, interface: &mut PlatformInterface, delta: f64) {
}

pub fn draw(state: &mut GameState, interface: &mut PlatformInterface, time: f64) {
    interface.set_background(20);
	state.field.draw(interface, BLOCK_SCALE);
}