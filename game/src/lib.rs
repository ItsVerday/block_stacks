use common::*;

pub struct GameState {
	pub pos: (f64, f64),
	pub vel: (f64, f64),
	pub radius: f64
}

pub fn requested_size() -> (u32, u32) {
    (320, 240)
}

pub fn requested_tickrate() -> u32 {
    120
}

pub fn init(interface: &mut PlatformInterface) -> GameState {
    interface.set_palette_color(0, [0, 0, 0, 255]);
    interface.set_palette_color(1, [255, 0, 0, 255]);

    GameState {
		pos: (interface.width as f64 / 2.0, interface.height as f64 / 2.0),
		vel: (45.0, 32.0),
		radius: 5.0
	}
}

pub fn tick(state: &mut GameState, interface: &mut PlatformInterface, delta: f64) {
	state.pos.0 += state.vel.0 * delta;
	state.pos.1 += state.vel.1 * delta;

	if state.pos.0 < state.radius {
		state.vel.0 = state.vel.0.abs();
	}

	if state.pos.0 > interface.width as f64 - state.radius {
		state.vel.0 = -state.vel.0.abs();
	}

	if state.pos.1 < state.radius {
		state.vel.1 = state.vel.1.abs();
	}

	if state.pos.1 > interface.height as f64 - state.radius {
		state.vel.1 = -state.vel.1.abs();
	}

	if let Some(mouse_pos) = interface.mouse_pos {
		if interface.input(Button::MouseLeft) == InputState::Pressed {
			state.pos = mouse_pos;
		}
	}

	if interface.input(Button::KeyW).is_down() {
		state.pos.1 -= 20.0 * delta;
	}

	if interface.input(Button::KeyA).is_down() {
		state.pos.0 -= 20.0 * delta;
	}

	if interface.input(Button::KeyS).is_down() {
		state.pos.1 += 20.0 * delta;
	}

	if interface.input(Button::KeyD).is_down() {
		state.pos.0 += 20.0 * delta;
	}
}

pub fn draw(state: &mut GameState, interface: &mut PlatformInterface, time: f64) {
    interface.set_background(0);

	for x in (state.pos.0 - state.radius).floor() as i32..(state.pos.0 + state.radius).ceil() as i32 {
		for y in (state.pos.1 - state.radius).floor() as i32..(state.pos.1 + state.radius).ceil() as i32 {
			let dx = x as f64 - state.pos.0;
			let dy = y as f64 - state.pos.1;
			if dx * dx + dy * dy < state.radius * state.radius {
				interface.set_pixel(x as f64, y as f64, 1);
			}
		}
	}
}