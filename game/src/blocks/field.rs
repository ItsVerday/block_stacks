use std::vec;

use super::{column::Column, block::Block, block_type::BlockType};
use common::PlatformInterface;

pub struct Field {
    pub width: u32,
    pub height: u32,

    pub columns: Vec<Column>
}

impl Field {
    pub fn new(interface: &mut PlatformInterface, width: u32, height: u32) -> Field {
        let mut columns = vec![];

        for x in 0..width {
            let mut column = Column::new(x, height);
            for _ in 0..10 {
                let block = Block::new(0, 0.0, false, BlockType::random_type(interface));
                column.stack_block_grounded(block);
            }

            columns.push(column);
        }

        Field {
            width,
            height,
            columns
        }
    }

    pub fn draw(&mut self, interface: &mut PlatformInterface, scale: f64) {
        for column in self.columns.iter_mut() {
            column.draw(interface, scale);
        }
    }
}