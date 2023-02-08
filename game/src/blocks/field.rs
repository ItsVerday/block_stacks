use std::{vec, collections::HashMap};

use crate::SPAWN_TIMER;

use super::{column::Column, block_kind::BlockKind};
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
		let mut previous_kinds = vec![];
		let mut current_kinds = vec![];

        for x in 0..width {
            let mut column = Column::new(x, height);
            for y in 0..4 {
				let mut kind = BlockKind::random_kind(interface);
				while Some(&kind) == current_kinds.last() || Some(&kind) == previous_kinds.get(y) {
					kind = BlockKind::random_kind(interface);
				}

				current_kinds.push(kind);
                let block = column.create_block(y as f64, true, kind);
                column.stack_block_grounded(block);
            }

            columns.push(column);
			previous_kinds = current_kinds;
			current_kinds = vec![];
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

		for (x, y) in check_blocks.iter() {
			self.check_match(*x as i32, *y as i32);
		}
	}

    pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        for column in self.columns.iter_mut() {
            column.draw(interface, time, scale);
        }
    }

	pub fn check_match(&mut self, x: i32, y: i32) {
		let kind = self.get_kind_at(x, y);
		let kind = match kind {
			Some(kind) => kind,
			None => return
		};

		let mut match_map = HashMap::new();
		let mut check_blocks = vec![];
		let mut matched_count = 0;

		let pos = (x, y);
		check_blocks.push(pos);
		while check_blocks.len() > 0 {
			let (check_x, check_y) = check_blocks.pop().unwrap();
			if match_map.contains_key(&(check_x, check_y)) {
				continue;
			}

			let kind_to_match = self.get_kind_at(check_x, check_y);
			let matches = match kind_to_match {
				Some(kind_to_match) => kind.matches(kind_to_match),
				None => false
			};

			match_map.insert((check_x, check_y), matches);
			if matches {
				matched_count += 1;
				if check_x > 0 {
					let pos = (check_x - 1, check_y);
					check_blocks.push(pos);
				}

				if check_x < self.width as i32 - 1 {
					let pos = (check_x + 1, check_y);
					check_blocks.push(pos);
				}

				if check_y > 0 {
					let pos = (check_x, check_y - 1);
					check_blocks.push(pos);
				}

				if check_y < self.height as i32 - 1 {
					let pos = (check_x, check_y + 1);
					check_blocks.push(pos);
				}
			}
		}

		if matched_count >= kind.minimum_clear_count() {
			for ((x, y), matches) in match_map.iter() {
				if !matches {
					continue;
				}

				let column = &mut self.columns[*x as usize];
				let block = &mut column.grounded_blocks[*y as usize];
				block.clear_timer = Some(0.15);
			}
		}
	}

	pub fn get_kind_at(&self, x: i32, y: i32) -> Option<BlockKind> {
		if x < 0 || x >= self.width as i32 {
			return None;
		}

		if y < 0 || y >= self.height as i32 {
			return None;
		}

		let column = &self.columns[x as usize];
		let block = column.grounded_blocks.get(y as usize);
		match block {
			Some(block) => match block.clear_timer {
				Some(timer) => None,
				None => Some(block.kind)
			}
			None => None
		}
	}

	pub fn get_valid_block_kind_for_column(&mut self, interface: &mut PlatformInterface, x: u32) -> BlockKind {
		let column = &self.columns[x as usize];
		let column_height = column.grounded_blocks.len() + column.falling_blocks.len();
		let top_block_kind = match column.falling_blocks.last() {
			Some(block) => Some(block.kind),
			None => match column.grounded_blocks.last() {
				Some(block) => Some(block.kind),
				None => None
			}
		};

		let left_block_kind = if x > 0 {self.get_column_block_kind_at_height(x - 1, column_height as u32)} else {None};
		let right_block_kind = if x < self.width - 1 {self.get_column_block_kind_at_height(x + 1, column_height as u32)} else {None};

		let mut kind = BlockKind::random_kind(interface);
		while Some(kind) == top_block_kind || Some(kind) == left_block_kind || Some(kind) == right_block_kind {
			kind = BlockKind::random_kind(interface);
		}

		kind
	}

	pub fn get_column_block_kind_at_height(&mut self, x: u32, height: u32) -> Option<BlockKind> {
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

			let x = interface.rng.gen_range(0..self.width);
			let kind = self.get_valid_block_kind_for_column(interface, x);
			let column = &mut self.columns[x as usize];
			let block = column.create_block(self.height as f64 + 4.0, false, kind);
			column.drop_block(block, self.height as f64 + 4.0);
		}
	}
}