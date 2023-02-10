use common::PlatformInterface;

use crate::{BLOCK_SCALE, GRAVITY_FACTOR, render::basic, PADDING};

use super::block_kind::BlockKind;

pub struct Block {
    pub x: u32,
    pub y: f64,
	pub y_velocity: f64,
    pub grounded: bool,

    pub kind: BlockKind,
    pub clear_timer: Option<f64>
}

impl Block {
    pub fn new(x: u32, y: f64, grounded: bool, kind: BlockKind) -> Block {
        Block {
            x,
            y,
			y_velocity: 0.0,
            grounded,
            kind,
            clear_timer: None
        }
    }

	pub fn tick(&mut self, _interface: &mut PlatformInterface, delta: f64) {
		if !self.grounded {
			self.y_velocity += GRAVITY_FACTOR * delta;
			self.y -= self.y_velocity * delta;
		}

        if let Some(timer) = self.clear_timer {
            self.clear_timer = Some(timer - delta);
        }
	}

    pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        match self.clear_timer {
            None => self.draw_kind(interface, time, scale),
            Some(timer) => {
                if timer < 0.05 {
                    basic::draw_rectangle(interface,
                        self.x as f64 * scale + PADDING,
                        interface.height as f64 - BLOCK_SCALE - self.y * scale - PADDING,
                        BLOCK_SCALE as u32, BLOCK_SCALE as u32, 15);
                } else if timer < 0.15 {
                    basic::draw_bordered_rectangle(interface,
                        self.x as f64 * scale + PADDING,
                        interface.height as f64 - BLOCK_SCALE - self.y * scale - PADDING,
                        BLOCK_SCALE as u32, BLOCK_SCALE as u32, 15, 16);
                } else {
                    self.draw_kind(interface, time, scale);
                }
            }
        }
    }

    pub fn draw_kind(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        self.kind.draw_instance(interface, time, self.x as f64 * scale, interface.height as f64 - BLOCK_SCALE - self.y * scale);
    }
}