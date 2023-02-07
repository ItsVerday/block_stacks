use common::PlatformInterface;

use crate::{FIELD_HEIGHT, FIELD_WIDTH, BLOCK_SCALE, PADDING};

pub struct Cursor {
	pub x: u32,
	pub y: u32,

	pub x_offset: f64,
	pub y_offset: f64,
	pub rotate_offset: f64
}

impl Cursor {
	pub fn new(x: u32, y: u32) -> Cursor {
		Cursor {
			x,
			y,
			x_offset: 0.0,
			y_offset: 0.0,
			rotate_offset: 0.0
		}
	}

	pub fn tick(&mut self, interface: &mut PlatformInterface, delta: f64) {
		if interface.input_pressed(common::Button::KeyUp) {
			self.y_offset = -1.0;
			if self.y < FIELD_HEIGHT - 2 {
				self.y += 1;
			}
		}

		if interface.input_pressed(common::Button::KeyLeft) {
			self.x_offset = -1.0;
			if self.x > 0 {
				self.x -= 1;
			}
		}
		
		if interface.input_pressed(common::Button::KeyDown) {
			self.y_offset = 1.0;
			if self.y > 0 {
				self.y -= 1;
			}
		}

		if interface.input_pressed(common::Button::KeyRight) {
			self.x_offset = 1.0;
			if self.x < FIELD_WIDTH - 2 {
				self.x += 1;
			}
		}

		self.x_offset *= 0.1_f64.powf(delta * 3.0);
		self.y_offset *= 0.1_f64.powf(delta * 3.0);
	}

	pub fn draw_corner(&mut self, interface: &mut PlatformInterface, x: f64, y: f64, x_pos: bool, y_pos: bool) {
		interface.set_pixel(x + 0.5 + PADDING, y + 0.5 - PADDING, 15);
		for i in 1..5 {
			interface.set_pixel(x + (i * if x_pos {1} else {-1}) as f64 + 0.5 + PADDING, y + 0.5 - PADDING, 15);
			interface.set_pixel(x + 0.5 + PADDING, y + (i * if y_pos {1} else {-1}) as f64 + 0.5 - PADDING, 15);
		}
	}

	pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
		let animation_offset = if time % 1.5 < 0.75 {0.0} else {1.0};

		let block_x = self.x as f64 * BLOCK_SCALE - 1.0;
		let block_y = interface.height as f64 - self.y as f64 * BLOCK_SCALE;

		self.draw_corner(interface, block_x + self.x_offset - animation_offset, block_y + self.y_offset + animation_offset, true, false);
		self.draw_corner(interface, block_x + self.x_offset - animation_offset, block_y - BLOCK_SCALE * 2.0 - 1.0 + self.y_offset - animation_offset, true, true);
		self.draw_corner(interface, block_x + self.x_offset + animation_offset + BLOCK_SCALE * 2.0 + 1.0, block_y + self.y_offset + animation_offset, false, false);
		self.draw_corner(interface, block_x + self.x_offset + animation_offset + BLOCK_SCALE * 2.0 + 1.0, block_y - BLOCK_SCALE * 2.0 - 1.0 + self.y_offset - animation_offset, false, true);
	}
}