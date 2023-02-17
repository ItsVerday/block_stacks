use std::{vec, collections::HashMap};

use crate::{ROTATE_CLOCKWISE_BUTTON, ROTATE_COUNTER_CLOCKWISE_BUTTON, data::{TickResult, Stats}};

use super::{column::Column, block_kind::BlockKind, cursor::Cursor, block::Block};
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

	pub fn tick(&mut self, interface: &mut PlatformInterface, cursor: &Cursor, delta: f64, stats: &Stats) -> TickResult {
		let mut result = TickResult {
			blocks_cleared: 0,
			score_gained: 0
		};

		self.handle_spawning(interface, delta, stats);

		let clockwise = interface.input_pressed(ROTATE_CLOCKWISE_BUTTON);
		let counter_clockwise = interface.input_pressed(ROTATE_COUNTER_CLOCKWISE_BUTTON);

		if clockwise && !counter_clockwise {
			self.rotate_blocks(cursor, true);
			interface.play_sound("input_rotate");
		}

		if counter_clockwise && !clockwise {
			self.rotate_blocks(cursor, false);
			interface.play_sound("input_rotate");
		}

		let mut check_blocks = vec![];

		for column in self.columns.iter_mut() {
            if column.tick(interface, delta, stats, &mut result) {
				for height in 0..column.grounded_blocks.len() {
					check_blocks.push((column.x, height));
				}
			}
        }

		for (x, y) in check_blocks.iter() {
			self.check_match(*x as i32, *y as i32);
		}

		result
	}

    pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        for column in self.columns.iter_mut() {
            column.draw(interface, time, scale);
        }
    }

	pub fn handle_spawning(&mut self, interface: &mut PlatformInterface, delta: f64, stats: &Stats) {
		self.spawn_timer += delta;

		while self.spawn_timer >= stats.spawn_timer {
			self.spawn_timer -= stats.spawn_timer;

			let x = interface.rng.gen_range(0..self.width);
			let kind = self.get_valid_block_kind_for_column(interface, x);
			let column = &mut self.columns[x as usize];
			let block = column.create_block(self.height as f64 + 4.0, false, kind);
			column.drop_block(block, self.height as f64 + 4.0);
		}
	}

	pub fn rotate_blocks(&mut self, cursor: &Cursor, is_clockwise: bool) {
		let cursor_x = cursor.x;
		let cursor_y = cursor.y;

		let old_block_00 = self.take_any_block_at(cursor_x as i32, cursor_y as i32);
		let old_block_10 = self.take_any_block_at(cursor_x as i32 + 1, cursor_y as i32);
		let old_block_01 = self.take_any_block_at(cursor_x as i32, cursor_y as i32 + 1);
		let old_block_11 = self.take_any_block_at(cursor_x as i32 + 1, cursor_y as i32 + 1);

		let (mut block_00, mut block_01, mut block_10, mut block_11) = (old_block_00, old_block_01, old_block_10, old_block_11);

		if is_clockwise {
			(block_00, block_01, block_10, block_11) = (block_10, block_00, block_11, block_01);
		} else {
			(block_00, block_01, block_10, block_11) = (block_01, block_11, block_00, block_10);
		}

		if let Some(block) = block_00 {
			self.insert_block_at(block, cursor_x, cursor_y);	
		}

		if let Some(block) = block_10 {
			self.insert_block_at(block, cursor_x + 1, cursor_y);	
		}

		if let Some(block) = block_01 {
			self.insert_block_at(block, cursor_x, cursor_y + 1);	
		}

		if let Some(block) = block_11 {
			self.insert_block_at(block, cursor_x + 1, cursor_y + 1);	
		}

		self.fix_column(cursor_x as i32);
		self.fix_column(cursor_x as i32 + 1);

		self.check_match(cursor_x as i32, cursor_y as i32);
		self.check_match(cursor_x as i32 + 1, cursor_y as i32);
		self.check_match(cursor_x as i32, cursor_y as i32 + 1);
		self.check_match(cursor_x as i32 + 1, cursor_y as i32 + 1);
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

		let mut clear_index = 0;
		let minimum_clear_count = kind.minimum_clear_count();

		if matched_count >= minimum_clear_count {
			for ((x, y), matches) in match_map.iter() {
				if !matches {
					continue;
				}

				let column = &mut self.columns[*x as usize];
				let block = &mut column.grounded_blocks[*y as usize];
				block.clear_timer = Some(0.15);
				if clear_index < minimum_clear_count {
					block.clear_score = 10;
				} else {
					let extra_blocks = clear_index - minimum_clear_count + 1;
					block.clear_score = (1.5_f64.powi(extra_blocks as i32).floor() as u64) * 5 + 10;
				}

				clear_index += 1;
			}
		}
	}

	pub fn take_any_block_at(&mut self, x: i32, y: i32) -> Option<Block> {
		if x < 0 || x >= self.width as i32 {
			return None;
		}

		if y < 0 || y >= self.height as i32 {
			return None;
		}

		let column = &mut self.columns[x as usize];

		let pop_index = 'idx: {
			for index in 0..column.grounded_blocks.len() {
				let block = &column.grounded_blocks[index];
				if (block.y - y as f64).abs() < 0.5 {
					break 'idx Some(index);
				}
			}

			None
		};

		if let Some(index) = pop_index {
			return Some(column.grounded_blocks.remove(index));
		}

		let pop_index = 'idx: {
			for index in 0..column.falling_blocks.len() {
				let block = &column.falling_blocks[index];
				if (block.y - y as f64).abs() < 0.5 {
					break 'idx Some(index);
				}
			}

			None
		};

		if let Some(index) = pop_index {
			return Some(column.falling_blocks.remove(index));
		}

		None
	}

	pub fn insert_block_at(&mut self, mut block: Block, x: u32, y: u32) {
		if x >= self.width as u32 {
			return;
		}

		if y >= self.height as u32 {
			return;
		}

		block.x = x;
		block.y = y as f64;
		block.y_velocity = 0.0;

		let column = &mut self.columns[x as usize];
		let insert_index = 'idx: {
			for index in 0..=column.grounded_blocks.len() {
				if (index >= column.grounded_blocks.len() || column.grounded_blocks[index].y > y as f64)
					&& (index == 0 || column.grounded_blocks[index - 1].y < y as f64) {
					break 'idx Some(index);
				}
			}

			None
		};

		if let Some(index) = insert_index {
			block.grounded = true;
			column.grounded_blocks.insert(index, block);
			return;
		}

		block.grounded = false;
		column.falling_blocks.push(block);
	}

	pub fn fix_column(&mut self, x: i32) {
		let column = &mut self.columns[x as usize];

		let fall_index = 'idx: {
			let mut prev_y = -1.0;
			for index in 0..column.grounded_blocks.len() {
				let block = &column.grounded_blocks[index];
				let is_valid = (prev_y + 1.0 - block.y).abs() < 0.01;
				prev_y = block.y;
				if !is_valid {
					break 'idx Some(index);
				}
			}

			None
		};

		if let Some(index) = fall_index {
			while column.grounded_blocks.len() > index {
				let mut block = column.grounded_blocks.remove(index);
				block.grounded = false;
				column.falling_blocks.push(block);
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
				Some(_) => None,
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
}