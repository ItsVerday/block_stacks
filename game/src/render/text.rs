use common::PlatformInterface;

const GLYPH_COUNT: usize = 16;
static GLYPH_CHARS: [char; GLYPH_COUNT] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P'];
static GLYPH_PIXELS: [[u8; 5]; GLYPH_COUNT] = [
    [0b1111110, 0b1000101, 0b1000101, 0b1111110, 0b0000000], // A
    [0b1111111, 0b1100101, 0b1100101, 0b1011010, 0b0000000], // B
    [0b1011110, 0b1100001, 0b1100001, 0b1010010, 0b0000000], // C
    [0b1111111, 0b1100001, 0b1100001, 0b1011110, 0b0000000], // D
    [0b1111111, 0b1100101, 0b1100101, 0b1100001, 0b0000000], // E
    [0b1111111, 0b1000101, 0b1000101, 0b1000001, 0b0000000], // F
    [0b1011110, 0b1100001, 0b1100101, 0b1011101, 0b0000000], // G
    [0b1111111, 0b1000100, 0b1000100, 0b1111111, 0b0000000], // H
    [0b1100001, 0b1111111, 0b1100001, 0b0000000, 0b0000000], // I
    [0b1010000, 0b1100000, 0b1100000, 0b1011111, 0b0000000], // J
    [0b1111111, 0b1000100, 0b1000100, 0b1111011, 0b0000000], // K
    [0b1111111, 0b1100000, 0b1100000, 0b1100000, 0b0000000], // L
    [0b1111111, 0b1000010, 0b1000100, 0b1000010, 0b1111111], // M
    [0b1111111, 0b1000010, 0b1000100, 0b1111111, 0b0000000], // N
    [0b1011110, 0b1100001, 0b1100001, 0b1011110, 0b0000000], // O
    [0b1111111, 0b1000101, 0b1000101, 0b1000010, 0b0000000]  // P
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