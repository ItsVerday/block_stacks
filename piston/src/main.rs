extern crate piston_window;
extern crate image;

use std::{time::Instant, collections::HashMap};

use common::PlatformInterface;
use game::*;
use piston_window::*;

fn main() {
	let size = requested_size();
	let tickrate = requested_tickrate();

	let mut interface = PlatformInterface::new(size.0 as usize, size.1 as usize);

    let mut window: PistonWindow =
        WindowSettings::new("Block Stacks", [1280, 960])
		.resizable(true)
		.graphics_api(OpenGL::V3_3)
		.build().unwrap();
	
	let mut canvas = image::ImageBuffer::new(interface.width as u32, interface.height as u32);
	let mut texture_context = TextureContext {
		factory: window.factory.clone(),
		encoder: window.factory.create_command_buffer().into()
	};
	let mut texture: G2dTexture = Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new().filter(Filter::Nearest)).unwrap();

	let mut game_state = init(&mut interface);

	let mut time = Instant::now();
	let mut ticks_executed = 0.0;
	let mut accum_time = 0.0;
    while let Some(event) = window.next() {
		let interface_width = size.0 as f64;
		let interface_height = size.1 as f64;

		if let Some(button) = event.press_args() {
			if let Some(button) = button_piston_to_common(button) {
				interface.inputs.insert(button, common::ButtonState::Pressed);
			}
		}

		if let Some(button) = event.release_args() {
			if let Some(button) = button_piston_to_common(button) {
				interface.inputs.insert(button, common::ButtonState::Released);
			}
		}

		if let Some(pos) = event.mouse_cursor_args() {
			let mut mouse_x = *pos.get(0).unwrap_or(&0.0);
			let mut mouse_y = *pos.get(1).unwrap_or(&0.0);
			let Size { width, height } = window.draw_size();

			let width_scale = width / interface_width;
			let height_scale = height / interface_height;

			if width_scale < height_scale {
				mouse_y -= (height - interface_height * width_scale) / 2.0;
				mouse_x /= width_scale;
				mouse_y /= width_scale;
			} else {
				mouse_x -= (width - interface_width * height_scale) / 2.0;
				mouse_x /= height_scale;
				mouse_y /= height_scale;
			}

			interface.mouse_pos = Some((mouse_x, mouse_y));
		}

		if event.render_args().is_some() {
			let now = Instant::now();
			let delta = now - time;
			time = now;
			
			accum_time += delta.as_secs_f64() * tickrate as f64;

			while accum_time >= ticks_executed {
				ticks_executed += 1.0;

				tick(&mut game_state, &mut interface, 1.0 / tickrate as f64);
				let mut new_inputs = HashMap::new();

				for (button, state) in interface.inputs.iter() {
					new_inputs.insert(*button, state.advance());
				}

				interface.inputs = new_inputs;
			}

			draw(&mut game_state, &mut interface, accum_time / tickrate as f64);

			for x in 0..interface.width {
				for y in 0..interface.height {
					let palette_index = interface.pixel_buffer[x + y * interface.width] as usize;
					canvas.put_pixel(x as u32, y as u32, image::Rgba(interface.palette[palette_index]));
				}
			}

			texture.update(&mut texture_context, &canvas).unwrap();
			window.draw_2d(&event, |context, graphics, device| {
				texture_context.encoder.flush(device);
				let [width, height] = context.get_view_size();
				clear([0.0, 0.0, 0.0, 1.0], graphics);
				
				let width_scale = width / interface_width;
				let height_scale = height / interface_height;

				let mut transform = context.transform;
				if width_scale < height_scale {
					transform = transform.trans(0.0, (height - interface_height * width_scale) / 2.0).scale(width_scale, width_scale);
				} else {
					transform = transform.trans((width - interface_width * height_scale) / 2.0, 0.0).scale(height_scale, height_scale);
				}

				image(&texture, transform, graphics);
			});
		}
    }
}

fn button_piston_to_common(button: Button) -> Option<common::Button> {
	use common::Button as B;
	match button {
		Button::Keyboard(key) => match key {
			Key::Backspace => Some(B::KeyBackspace),
			Key::Tab => Some(B::KeyTab),
			Key::Return => Some(B::KeyReturn),
			Key::Escape => Some(B::KeyEscape),
			Key::Space => Some(B::KeySpace),
			Key::Exclaim => Some(B::KeyExclaim),
			Key::Quotedbl => Some(B::KeyDoubleQuote),
			Key::Hash => Some(B::KeyHash),
			Key::Dollar => Some(B::KeyDollar),
			Key::Percent => Some(B::KeyPercent),
			Key::Ampersand => Some(B::KeyAmpersand),
			Key::Quote => Some(B::KeyQuote),
			Key::LeftParen => Some(B::KeyLeftParen),
			Key::RightParen => Some(B::KeyRightParen),
			Key::Asterisk => Some(B::KeyAsterisk),
			Key::Plus => Some(B::KeyPlus),
			Key::Comma => Some(B::KeyComma),
			Key::Minus => Some(B::KeyMinus),
			Key::Period => Some(B::KeyPeriod),
			Key::Slash => Some(B::KeySlash),
			Key::D0 => Some(B::Key0),
			Key::D1 => Some(B::Key1),
			Key::D2 => Some(B::Key2),
			Key::D3 => Some(B::Key3),
			Key::D4 => Some(B::Key4),
			Key::D5 => Some(B::Key5),
			Key::D6 => Some(B::Key6),
			Key::D7 => Some(B::Key7),
			Key::D8 => Some(B::Key8),
			Key::D9 => Some(B::Key9),
			Key::Colon => Some(B::KeyColon),
			Key::Semicolon => Some(B::KeySemicolon),
			Key::Less => Some(B::KeyLess),
			Key::Equals => Some(B::KeyEquals),
			Key::Greater => Some(B::KeyGreater),
			Key::Question => Some(B::KeyQuestion),
			Key::At => Some(B::KeyAt),
			Key::LeftBracket => Some(B::KeyLeftBracket),
			Key::Backslash => Some(B::KeyBackslash),
			Key::RightBracket => Some(B::KeyRightBracket),
			Key::Caret => Some(B::KeyCaret),
			Key::Underscore => Some(B::KeyUnderscore),
			Key::Backquote => Some(B::KeyBackquote),
			Key::A => Some(B::KeyA),
			Key::B => Some(B::KeyB),
			Key::C => Some(B::KeyC),
			Key::D => Some(B::KeyD),
			Key::E => Some(B::KeyE),
			Key::F => Some(B::KeyF),
			Key::G => Some(B::KeyG),
			Key::H => Some(B::KeyH),
			Key::I => Some(B::KeyI),
			Key::J => Some(B::KeyJ),
			Key::K => Some(B::KeyK),
			Key::L => Some(B::KeyL),
			Key::M => Some(B::KeyM),
			Key::N => Some(B::KeyN),
			Key::O => Some(B::KeyO),
			Key::P => Some(B::KeyP),
			Key::Q => Some(B::KeyQ),
			Key::R => Some(B::KeyR),
			Key::S => Some(B::KeyS),
			Key::T => Some(B::KeyT),
			Key::U => Some(B::KeyU),
			Key::V => Some(B::KeyV),
			Key::W => Some(B::KeyW),
			Key::X => Some(B::KeyX),
			Key::Y => Some(B::KeyY),
			Key::Z => Some(B::KeyZ),
			Key::Delete => Some(B::KeyDelete),
			_ => None
		},
		Button::Mouse(mouse_buton) => match mouse_buton {
			MouseButton::Left => Some(B::MouseLeft),
			MouseButton::Right => Some(B::MouseRight),
			MouseButton::Middle => Some(B::MouseMiddle),
			_ => None
		},
		_ => None
	}
}