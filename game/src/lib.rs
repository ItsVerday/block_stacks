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

pub mod stats {
	pub mod stat;
}

pub mod data;

use common::*;
use blocks::{field::Field, cursor::Cursor};
use data::{blocks_for_level, Stats};

// Game constants
const FIELD_WIDTH: u32 = 10;
const FIELD_HEIGHT: u32 = 20;
const BLOCK_SCALE: f64 = 10.0;
const PADDING: f64 = 20.0;

// Controls
const UP_BUTTON: Button = Button::KeyW;
const DOWN_BUTTON: Button = Button::KeyS;
const LEFT_BUTTON: Button = Button::KeyA;
const RIGHT_BUTTON: Button = Button::KeyD;
const ROTATE_CLOCKWISE_BUTTON: Button = Button::MouseLeft;
const ROTATE_COUNTER_CLOCKWISE_BUTTON: Button = Button::MouseRight;

pub struct GameState {
	pub field: Field,
	pub cursor: Cursor,
	pub blocks_cleared: u32,
	pub score: u64,
	pub level: u16,
	pub blocks_to_next_level: i32
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
		cursor: Cursor::new(FIELD_WIDTH / 2 - 1, 2),
		blocks_cleared: 0,
		score: 0,
		level: 100,
		blocks_to_next_level: blocks_for_level(100)
	}
}

pub fn tick(state: &mut GameState, interface: &mut PlatformInterface, delta: f64) {
	let mut game_stats = Stats::from(state.level);

	state.cursor.tick(interface, delta);
 	let result = state.field.tick(interface, &state.cursor, delta, &mut game_stats);
	state.blocks_cleared += result.blocks_cleared;
	state.score += result.score_gained;

	state.blocks_to_next_level -= result.blocks_cleared as i32;
	while state.blocks_to_next_level <= 0 {
		state.level += 1;
		state.blocks_to_next_level += blocks_for_level(state.level);
	}
}

pub fn draw(state: &mut GameState, interface: &mut PlatformInterface, time: f64) {
    interface.set_background(20);
	state.field.draw(interface, time, BLOCK_SCALE);
	state.cursor.draw(interface, time, BLOCK_SCALE);

	text!(interface, 132.0, 182.0, 15; "SCORE: {}", state.score);
	text!(interface, 132.0, 192.0, 15; "LEVEL: {}", state.level);
	text!(interface, 132.0, 202.0, 15; "BLOCKS CLEARED: {}", state.blocks_cleared);
	text!(interface, 132.0, 212.0, 15; "BLOCKS TO LEVEL UP: {}", state.blocks_to_next_level);
}