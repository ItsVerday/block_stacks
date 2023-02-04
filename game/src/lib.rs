use std::f64::consts::PI;

use common::*;
use rand::Rng;

pub struct Ball {
	pub pos: (f64, f64),
	pub vel: (f64, f64),
	pub radius: f64,
	pub color: u8
}

pub struct GameState {
	pub balls: Vec<Ball>
}

pub fn requested_size() -> (u32, u32) {
    (360, 240)
}

pub fn requested_tickrate() -> u32 {
    60
}

pub fn init(interface: &mut PlatformInterface) -> GameState {
    interface.set_palette_color(0, [64, 64, 64, 255]);
    interface.set_palette_color(1, [255, 0, 0, 255]);
    interface.set_palette_color(2, [255, 255, 0, 255]);
    interface.set_palette_color(3, [0, 255, 0, 255]);
    interface.set_palette_color(4, [0, 255, 255, 255]);
    interface.set_palette_color(5, [0, 0, 255, 255]);
    interface.set_palette_color(6, [255, 0, 255, 255]);
    interface.set_palette_color(7, [255, 255, 255, 255]);

	let mut balls = vec![];

	for _ in 0..100 {
		let radius = interface.rng.gen_range(3.0..10.0);
		let direction = interface.rng.gen_range(0.0..PI * 2.0);
		let speed = interface.rng.gen_range(10.0..100.0);

		balls.push(Ball {
			pos: (interface.rng.gen_range(radius..interface.width as f64 - radius), interface.rng.gen_range(radius..interface.height as f64 - radius)),
			vel: (direction.sin() * speed, direction.cos() * speed),
			radius,
			color: interface.rng.gen_range(1..8)
		});
	}

    GameState {
		balls
	}
}

pub fn tick(state: &mut GameState, interface: &mut PlatformInterface, delta: f64) {
	for ball in state.balls.iter_mut() {
		ball.pos.0 += ball.vel.0 * delta;
		ball.pos.1 += ball.vel.1 * delta;

		if ball.pos.0 < ball.radius {
			ball.vel.0 = ball.vel.0.abs();
		}

		if ball.pos.0 > interface.width as f64 - ball.radius {
			ball.vel.0 = -ball.vel.0.abs();
		}

		if ball.pos.1 < ball.radius {
			ball.vel.1 = ball.vel.1.abs();
		}

		if ball.pos.1 > interface.height as f64 - ball.radius {
			ball.vel.1 = -ball.vel.1.abs();
		}

		if let Some(mouse_pos) = interface.mouse_pos {
			if interface.input(Button::MouseLeft) == InputState::Pressed {
				ball.pos = mouse_pos;
			}
		}
	}
}

pub fn draw(state: &mut GameState, interface: &mut PlatformInterface, time: f64) {
    interface.set_background(0);
	for ball in state.balls.iter() {
		for x in (ball.pos.0 - ball.radius).floor() as i32..(ball.pos.0 + ball.radius).ceil() as i32 {
			for y in (ball.pos.1 - ball.radius).floor() as i32..(ball.pos.1 + ball.radius).ceil() as i32 {
				let dx = x as f64 - ball.pos.0;
				let dy = y as f64 - ball.pos.1;
				if dx * dx + dy * dy < ball.radius * ball.radius {
					interface.set_pixel(x as f64, y as f64, ball.color);
				}
			}
		}
	}
}