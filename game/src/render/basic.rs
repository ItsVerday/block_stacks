use common::PlatformInterface;

pub fn draw_rectangle(interface: &mut PlatformInterface, x: f64, y: f64, width: u32, height: u32, color: u8) {
    for x_offset in 0..width as u32 {
        for y_offset in 0..height as u32 {
            interface.set_pixel(x + x_offset as f64, y + y_offset as f64, color);
        }
    }
}

pub fn draw_bordered_rectangle(interface: &mut PlatformInterface, x: f64, y: f64, width: u32, height: u32, color1: u8, color2: u8) {
    for x_offset in 0..width as u32 {
        for y_offset in 0..height as u32 {
            let color = if x_offset == 0 || y_offset == 0 || x_offset == width - 1 || y_offset == height - 1 {color2} else {color1};
            interface.set_pixel(x + x_offset as f64, y + y_offset as f64, color);
        }
    }
}