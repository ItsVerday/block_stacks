use std::vec;

use crate::{FIELD_HEIGHT, SPAWN_TIMER, FIELD_WIDTH};

use super::{column::Column, block_type::BlockType};
use common::PlatformInterface;
use rand::Rng;

pub struct Field {
    pub width: u32,
    pub height: u32,

    pub columns: Vec<Column>,
	pub spawn_timer: f64
}

impl Field {
    pub fn new(interface: &mut PlatformInterface, width: u32, height: u32) -> Field {
        let mut columns = vec![];
		let mut previous_types = vec![];
		let mut current_types = vec![];

        for x in 0..width {
            let mut column = Column::new(x, height);
            for y in 0..4 {
				let mut kind = BlockType::random_type(interface);
				while Some(&kind) == current_types.last() || Some(&kind) == previous_types.get(y) {
					kind = BlockType::random_type(interface);
				}

				current_types.push(kind);
                let block = column.create_block(y as f64, true, kind);
                column.stack_block_grounded(block);
            }

            columns.push(column);
			previous_types = current_types;
			current_types = vec![];
        }

        Field {
            width,
            height,
            columns,
			spawn_timer: 0.0
        }
    }

	pub fn tick(&mut self, interface: &mut PlatformInterface, delta: f64) {
		self.handle_spawning(interface, delta);

		let mut check_blocks = vec![];

		for column in self.columns.iter_mut() {
            let blocks_to_check = column.tick(interface, delta);
			for height in blocks_to_check.into_iter() {
				check_blocks.push((column.x, height));
			}
        }
	}

    pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        for column in self.columns.iter_mut() {
            column.draw(interface, time, scale);
        }
    }

	pub fn get_valid_block_type_for_column(&mut self, interface: &mut PlatformInterface, x: u32) -> BlockType {
		let column = &self.columns[x as usize];
		let column_height = column.grounded_blocks.len() + column.falling_blocks.len();
		let top_block_type = match column.falling_blocks.last() {
			Some(block) => Some(block.kind),
			None => match column.grounded_blocks.last() {
				Some(block) => Some(block.kind),
				None => None
			}
		};

		let left_block_type = if x > 0 {self.get_column_block_type_at_height(x - 1, column_height as u32)} else {None};
		let right_block_type = if x < FIELD_WIDTH - 1 {self.get_column_block_type_at_height(x + 1, column_height as u32)} else {None};

		let mut kind = BlockType::random_type(interface);
		while Some(kind) == top_block_type || Some(kind) == left_block_type || Some(kind) == right_block_type {
			kind = BlockType::random_type(interface);
		}

		kind
	}

	pub fn get_column_block_type_at_height(&mut self, x: u32, height: u32) -> Option<BlockType> {
		let column = &self.columns[x as usize];

		let mut index = height;
		if column.grounded_blocks.len() > index as usize {
			return Some(column.grounded_blocks[index as usize].kind);
		} else {
			index -= column.grounded_blocks.len() as u32;
		}

		if column.falling_blocks.len() > index as usize {
			Some(column.falling_blocks[index as usize].kind)
		} else {
			None
		}
	}

	pub fn handle_spawning(&mut self, interface: &mut PlatformInterface, delta: f64) {
		self.spawn_timer += delta;

		while self.spawn_timer >= SPAWN_TIMER {
			self.spawn_timer -= SPAWN_TIMER;

			let x = interface.rng.gen_range(0..FIELD_WIDTH);
			let kind = self.get_valid_block_type_for_column(interface, x);
			let column = &mut self.columns[x as usize];
			let block = column.create_block(FIELD_HEIGHT as f64, false, kind);
			column.drop_block(block, FIELD_HEIGHT as f64);
		}
	}
}