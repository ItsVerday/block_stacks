use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::{stats::stat::Stat, blocks::block_kind::BlockKind};

pub struct TickResult {
    pub blocks_cleared: u32,
    pub score_gained: u64
}

pub fn blocks_for_level(level: u16) -> i32 {
    let level = level as i32 - 1;

    if level < 7 {
        level * 2 + 20
    } else if level < 15 {
        level * 5 - 15
    } else if level < 25 {
        level * 10 - 90
    } else if level < 40 {
        level * 15 - 210
    } else if level < 60 {
        level * 25 - 610
    } else if level < 80 {
        level * 40 - 1510
    } else {
        (169.0 * 1.03_f64.powi(level as i32 - 80)).round() as i32 * 10
    }
}

pub struct Stats {
    pub gravity: Stat,
    pub spawn_timer: Stat,
    pub base_points: Stat,
    pub block_kinds: HashMap<BlockKind, BlockKindStats>
}

impl Stats {
    pub fn from(level: u16) -> Stats {
        Stats {
            gravity: {
                let mut gravity = Stat::new(10.0);
                gravity.base.add(1.0 * (level as f64 - 1.0));
                gravity.total.hard_min(5.0);
                gravity.total.hard_max(200.0);
                gravity
            },
            spawn_timer: {
                let mut spawn_timer = Stat::new(1.5);
                spawn_timer.base.multiply(0.98_f64.powi(level as i32 - 1));
                spawn_timer.total.hard_min(0.25);
                spawn_timer.total.hard_max(4.0);
                spawn_timer
            },
            base_points: {
                let mut base_points = Stat::new(10.0);
                base_points.total.hard_min(5.0);
                base_points
            },
            block_kinds: {
                let mut block_kinds = HashMap::new();
                for kind in BlockKind::iter() {
                    block_kinds.insert(kind, BlockKindStats {
                        minimum_clear_count: {
                            let mut minimum_clear_count = Stat::new(3.0);
                            minimum_clear_count.total.hard_min(2.0);
                            minimum_clear_count
                        }
                    });
                }

                block_kinds
            }
        }
    }
}

pub struct BlockKindStats {
    pub minimum_clear_count: Stat
}