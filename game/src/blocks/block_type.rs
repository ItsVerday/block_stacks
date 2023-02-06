use common::PlatformInterface;
use rand::Rng;

use crate::BLOCK_SCALE;

pub enum BlockType {
    Fire,
    Ice,
    Plant,
    Water,
    Lightning,
    Magic
}

impl BlockType {
    pub fn random_type(interface: &mut PlatformInterface) -> BlockType {
        match interface.rng.gen_range(0..6) {
            0 => Self::Fire,
            1 => Self::Ice,
            2 => Self::Plant,
            3 => Self::Water,
            4 => Self::Lightning,
            5 => Self::Magic,
            _ => Self::Fire
        }
    }

    pub fn draw_bordered_rectangle(interface: &mut PlatformInterface, x: f64, y: f64, color1: u8, color2: u8) {
        for x_offset in 0..BLOCK_SCALE as u32 {
            for y_offset in 0..BLOCK_SCALE as u32 {
                let color = if x_offset == 0 || y_offset == 0 {color2} else {color1};
                interface.set_pixel(x + x_offset as f64, y + y_offset as f64, color);
            }
        }
    }

    pub fn draw_instance(&self, interface: &mut PlatformInterface, x: f64, y: f64) {
        match self {
            Self::Fire => BlockType::draw_bordered_rectangle(interface, x, y, 27, 28),
            Self::Ice => BlockType::draw_bordered_rectangle(interface, x, y, 14, 13),
            Self::Plant => BlockType::draw_bordered_rectangle(interface, x, y, 6, 7),
            Self::Water => BlockType::draw_bordered_rectangle(interface, x, y, 12, 11),
            Self::Lightning => BlockType::draw_bordered_rectangle(interface, x, y, 4, 3),
            Self::Magic => BlockType::draw_bordered_rectangle(interface, x, y, 33, 32)
        }
    }
}