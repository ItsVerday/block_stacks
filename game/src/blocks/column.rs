use std::vec;

use common::PlatformInterface;

use crate::FIELD_HEIGHT;

use super::{block::Block, block_type::BlockType};

pub struct Column {
    pub x: u32,
    pub height: u32,

    pub grounded_blocks: Vec<Block>,
    pub falling_blocks: Vec<Block>
}

impl Column {
    pub fn new(x: u32, height: u32) -> Column {
        let grounded = vec![];
        let falling = vec![];

        Column {
            x,
            height,
            grounded_blocks: grounded,
            falling_blocks: falling
        }
    }

	pub fn tick(&mut self, interface: &mut PlatformInterface, delta: f64) {
		let mut indices_to_remove = vec![];

		for index in 0..self.falling_blocks.len() {
			let block = &mut self.falling_blocks[index];
            block.tick(interface, delta);

			let block_y = block.y;
			let ground_height = self.get_ground_height();
			if block_y <= ground_height {
				indices_to_remove.push(index);
			}
        }

		indices_to_remove.reverse();

		for index in indices_to_remove.iter() {
			let block = self.falling_blocks.remove(*index);
			self.stack_block_grounded(block);
		}

		for block in self.grounded_blocks.iter_mut() {
            block.tick(interface, delta);
        }
	}

    pub fn draw(&mut self, interface: &mut PlatformInterface, time: f64, scale: f64) {
        for block in self.falling_blocks.iter_mut() {
            block.draw(interface, time, scale);
        }

        for block in self.grounded_blocks.iter_mut() {
            block.draw(interface, time, scale);
        }
    }

	pub fn create_block(&mut self, y: f64, grounded: bool, kind: BlockType) -> Block {
		let mut block = Block::new(self.x, y, grounded, kind);
		block
	}

    pub fn stack_block_grounded(&mut self, mut block: Block) {
        let grounded_y = self.get_ground_height();

        block.x = self.x;
        block.grounded = true;
        block.y = grounded_y;
        self.grounded_blocks.push(block);
    }

	pub fn drop_block(&mut self, mut block: Block, height: f64) {
		block.x = self.x;
		block.grounded = false;
		block.y = height;
		self.falling_blocks.push(block);
	}

    pub fn get_ground_height(&mut self) -> f64 {
        match self.grounded_blocks.last() {
            None => 0.0,
            Some(block) => block.y + 1.0
        }
    }
}