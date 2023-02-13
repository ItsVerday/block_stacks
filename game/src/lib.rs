pub mod render {
	pub mod palette;
	pub mod basic;
	pub mod text;
}

pub mod blocks {
	pub mod field;
	pub mod column;
	pub mod block;
	pub mod block_kind;

	pub mod cursor;
}

use common::*;
use blocks::{field::Field, cursor::Cursor};

// Game constants
const FIELD_WIDTH: u32 = 10;
const FIELD_HEIGHT: u32 = 20;
const BLOCK_SCALE: f64 = 10.0;
const GRAVITY_FACTOR: f64 = 10.0;
const PADDING: f64 = 20.0;
const SPAWN_TIMER: f64 = 1.5;

// Controls
const UP_BUTTON: Button = Button::KeyW;
const DOWN_BUTTON: Button = Button::KeyS;
const LEFT_BUTTON: Button = Button::KeyA;
const RIGHT_BUTTON: Button = Button::KeyD;
const ROTATE_CLOCKWISE_BUTTON: Button = Button::MouseLeft;
const ROTATE_COUNTER_CLOCKWISE_BUTTON: Button = Button::MouseRight;

pub struct GameState {
	pub field: Field,
	pub cursor: Cursor
}

pub fn requested_size() -> (u32, u32) {
    (360, 240)
}

pub fn requested_tickrate() -> u32 {
    60
}

pub fn init(interface: &mut PlatformInterface) -> GameState {
	render::palette::load_palette(interface);

    GameState {
		field: Field::new(interface, FIELD_WIDTH, FIELD_HEIGHT),
		cursor: Cursor::new(FIELD_WIDTH / 2 - 1, 2)
	}
}

pub fn tick(state: &mut GameState, interface: &mut PlatformInterface, delta: f64) {
	state.cursor.tick(interface, delta);
 	state.field.tick(interface, &state.cursor, delta);
}

pub fn draw(state: &mut GameState, interface: &mut PlatformInterface, time: f64) {
    interface.set_background(20);
	state.field.draw(interface, time, BLOCK_SCALE);
	state.cursor.draw(interface, time, BLOCK_SCALE);

	text!(interface, 2.0, 2.0, 15, "HELLO {}", "WORLD");
}