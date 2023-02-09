use common::PlatformInterface;
use rand::Rng;

use crate::{BLOCK_SCALE, PADDING, util};

#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
pub enum BlockKind {
    Fire,
    Ice,
    Plant,
    Water,
    Lightning,
    Magic
}

impl BlockKind {
    pub fn random_kind(interface: &mut PlatformInterface) -> BlockKind {
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
        util::draw_bordered_rectangle(interface, x + PADDING, y - PADDING, BLOCK_SCALE as u32, BLOCK_SCALE as u32, color1, color2);
    }

    pub fn draw_instance(&self, interface: &mut PlatformInterface, _time: f64, x: f64, y: f64) {
        match self {
            Self::Fire => BlockKind::draw_bordered_rectangle(interface, x, y, 27, 28),
            Self::Ice => BlockKind::draw_bordered_rectangle(interface, x, y, 14, 13),
            Self::Plant => BlockKind::draw_bordered_rectangle(interface, x, y, 6, 7),
            Self::Water => BlockKind::draw_bordered_rectangle(interface, x, y, 12, 11),
            Self::Lightning => BlockKind::draw_bordered_rectangle(interface, x, y, 4, 3),
            Self::Magic => BlockKind::draw_bordered_rectangle(interface, x, y, 33, 32)
        }
    }

    pub fn matches(&self, other: BlockKind) -> bool {
        match self {
            Self::Fire => other == Self::Fire,
            Self::Ice => other == Self::Ice,
            Self::Plant => other == Self::Plant,
            Self::Water => other == Self::Water,
            Self::Lightning => other == Self::Lightning,
            Self::Magic => other == Self::Magic
        }
    }

    pub fn minimum_clear_count(&self) -> u32 {
        3
    }
}