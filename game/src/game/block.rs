use common::PlatformInterface;

use crate::BLOCK_SCALE;

use super::block_type::BlockType;

pub struct Block {
    pub x: u32,
    pub y: f64,
    pub grounded: bool,

    pub kind: BlockType
}

impl Block {
    pub fn new(x: u32, y: f64, grounded: bool, kind: BlockType) -> Block {
        Block {
            x,
            y,
            grounded,
            kind
        }
    }

    pub fn draw(&mut self, interface: &mut PlatformInterface, scale: f64) {
        self.kind.draw_instance(interface, self.x as f64 * scale, interface.height as f64 - BLOCK_SCALE - self.y * scale);
    }
}