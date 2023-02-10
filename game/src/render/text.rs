use common::PlatformInterface;

const GLYPH_COUNT: usize = 1;
static GLYPH_CHARS: [char; GLYPH_COUNT] = ['A'];
static GLYPH_PIXELS: [[u8; 4]; GLYPH_COUNT] = [
    [0b1111110, 0b1000101, 0b1000101, 0b1111110] // A
];

pub fn find_char(c: char) -> Option<usize> {
    for index in 0..GLYPH_CHARS.len() {
        if c == GLYPH_CHARS[index] {
            return Some(index);
        }
    }

    None
}

pub fn draw_text(interface: &mut PlatformInterface, string: &str, x: f64, y: f64, color: u8) {
    let mut current_x = x;
    let current_y = y;

    for c in string.chars() {
        if let Some(index) = find_char(c) {
            let pixels = GLYPH_PIXELS[index];
            for column in pixels {
                if column & 0b1000000 == 0 {
                    current_x += 1.0;
                    break;
                }

                let mut mask = 0b1;
                for b in 0..6 {
                    if column & mask != 0 {
                        interface.set_pixel(current_x, current_y + b as f64, color);
                    }
                    
                    mask <<= 1;
                }

                current_x += 1.0;
            }
        }

        current_x += 1.0;
    }
}