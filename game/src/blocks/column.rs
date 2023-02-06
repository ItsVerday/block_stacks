use common::PlatformInterface;

use super::block::Block;

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

    pub fn draw(&mut self, interface: &mut PlatformInterface, scale: f64) {
        for block in self.falling_blocks.iter_mut() {
            block.draw(interface, scale);
        }

        for block in self.grounded_blocks.iter_mut() {
            block.draw(interface, scale);
        }
    }

    pub fn stack_block_grounded(&mut self, mut block: Block) {
        let grounded_y = self.get_ground_height();

        block.x = self.x;
        block.grounded = true;
        block.y = grounded_y;
        self.grounded_blocks.push(block);
    }

    pub fn get_ground_height(&mut self) -> f64 {
        match self.grounded_blocks.last() {
            None => 0.0,
            Some(block) => block.y + 1.0
        }
    }
}