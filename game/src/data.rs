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
    pub gravity: f64,
    pub spawn_timer: f64
}

impl Stats {
    pub fn from(level: u16) -> Stats {
        Stats {
            gravity: 10.0 + 1.0 * (level as f64 - 1.0),
            spawn_timer: 1.5 * 0.98_f64.powi(level as i32 - 1)
        }
    }
}