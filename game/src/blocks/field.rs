use std::vec;

use crate::FIELD_HEIGHT;

use super::{column::Column, block_type::BlockType};
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
            for y in 0..10 {
                let block = column.create_block(y as f64, true, BlockType::random_type(interface));
                column.stack_block_grounded(block);
            }

			let block = column.create_block(FIELD_HEIGHT as f64, false, BlockType::random_type(interface));
			column.drop_block(block, 15.0 + x as f64);
			let block = column.create_block(FIELD_HEIGHT as f64, false, BlockType::random_type(interface));
			column.drop_block(block, 20.0 + x as f64);
            columns.push(column);
        }

        Field {
            width,
            height,
            columns
        }
    }

	pub fn tick(&mut self, interface: &mut PlatformInterface, delta: f64) {
		for column in self.columns.iter_mut() {
            column.tick(interface, delta);
        }
	}

    pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        for column in self.columns.iter_mut() {
            column.draw(interface, time, scale);
        }
    }
}