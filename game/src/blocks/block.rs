use common::PlatformInterface;

use crate::{BLOCK_SCALE, GRAVITY_FACTOR};

use super::block_kind::BlockKind;

pub struct Block {
    pub x: u32,
    pub y: f64,
	pub y_velocity: f64,
    pub grounded: bool,

    pub kind: BlockKind
}

impl Block {
    pub fn new(x: u32, y: f64, grounded: bool, kind: BlockKind) -> Block {
        Block {
            x,
            y,
			y_velocity: 0.0,
            grounded,
            kind
        }
    }

	pub fn tick(&mut self, interface: &mut PlatformInterface, delta: f64) {
		if !self.grounded {
			self.y_velocity += GRAVITY_FACTOR * delta;
			self.y -= self.y_velocity * delta;
		}
	}

    pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        self.kind.draw_instance(interface, time, self.x as f64 * scale, interface.height as f64 - BLOCK_SCALE - self.y * scale);
    }
}