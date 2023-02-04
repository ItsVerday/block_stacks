extern crate image;
extern crate piston_window;

mod input;

use std::{collections::HashMap, time::Instant};

use common::{InputState, PlatformInterface};
use game::*;
use image::{ImageBuffer, Rgba};
use piston_window::*;
use rand::rngs::ThreadRng;

fn main() {
    let rng = rand::thread_rng();
    let mut state = create_state(rng);
    let mut game_state = game::init(&mut state.interface);

    let mut window: PistonWindow = WindowSettings::new("Block Stacks", [1440, 960])
        .resizable(true)
        .graphics_api(OpenGL::V3_3)
        .build()
        .unwrap();

    let mut texture_info = create_texture_info(&mut window, &state);
    event_loop(&mut state, &mut game_state, &mut texture_info, &mut window);
}

fn event_loop(
    state: &mut State,
    game_state: &mut GameState,
    texture_info: &mut TextureInfo,
    window: &mut PistonWindow,
) {
    while let Some(event) = window.next() {
        update_inputs(event.press_args(), state, common::InputState::Pressed);
        update_inputs(event.release_args(), state, common::InputState::Released);
        handle_mouse_move(event.mouse_cursor_args(), window.size(), state);

        if event.render_args().is_some() {
            do_loop(&event, state, game_state, texture_info, window);
        }
    }
}

fn update_inputs(args: Option<Button>, state: &mut State, new_state: InputState) {
    if let Some(button) = args {
        if let Some(button) = input::button_piston_to_common(button) {
            state.interface.inputs.insert(button, new_state);
        }
    }
}

fn handle_mouse_move(args: Option<[f64; 2]>, size: Size, state: &mut State) {
    let interface_width = state.size.0 as f64;
    let interface_height = state.size.1 as f64;

    if let Some(pos) = args {
        let mut mouse_x = pos[0];
        let mut mouse_y = pos[1];
        let Size { width, height } = size;

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

        state.interface.mouse_pos = Some((mouse_x, mouse_y));
    }
}

fn do_loop(
    event: &Event,
    state: &mut State,
    game_state: &mut GameState,
    texture_info: &mut TextureInfo,
    window: &mut PistonWindow,
) {
    do_ticks(state, game_state);
    game::draw(
        game_state,
        &mut state.interface,
        state.accum_time / state.tickrate as f64,
    );
    do_draw(event, state, texture_info, window);
}

fn do_ticks(state: &mut State, game_state: &mut GameState) {
    let now = Instant::now();
    let delta = now - state.time;
    state.time = now;

    state.accum_time += delta.as_secs_f64() * state.tickrate as f64;

    while state.accum_time >= state.ticks_executed {
        state.ticks_executed += 1.0;

        game::tick(
            game_state,
            &mut state.interface,
            1.0 / state.tickrate as f64,
        );
        let mut new_inputs = HashMap::new();

        for (button, state) in state.interface.inputs.iter() {
            new_inputs.insert(*button, state.advance());
        }

        state.interface.inputs = new_inputs;
    }
}

fn do_draw(
    event: &Event,
    state: &mut State,
    texture_info: &mut TextureInfo,
    window: &mut PistonWindow,
) {
    for x in 0..state.interface.width {
        for y in 0..state.interface.height {
            let palette_index =
                state.interface.pixel_buffer[x + y * state.interface.width] as usize;
            texture_info.buffer.put_pixel(
                x as u32,
                y as u32,
                image::Rgba(state.interface.palette[palette_index]),
            );
        }
    }

    texture_info
        .texture
        .update(&mut texture_info.context, &texture_info.buffer)
        .unwrap();
    window.draw_2d(event, |context, graphics, device| {
        texture_info.context.encoder.flush(device);
        let [width, height] = context.get_view_size();
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        let width_scale = width / state.size.0 as f64;
        let height_scale = height / state.size.1 as f64;

        let mut transform = context.transform;
        if width_scale < height_scale {
            transform = transform
                .trans(0.0, (height - state.size.1 as f64 * width_scale) / 2.0)
                .scale(width_scale, width_scale);
        } else {
            transform = transform
                .trans((width - state.size.0 as f64 * height_scale) / 2.0, 0.0)
                .scale(height_scale, height_scale);
        }

        image(&texture_info.texture, transform, graphics);
    });
}

struct State {
    size: (u32, u32),
    tickrate: u32,
    interface: PlatformInterface,
    time: Instant,
    ticks_executed: f64,
    accum_time: f64,
}

fn create_state(rng: ThreadRng) -> State {
    let interface_size = requested_size();
    let interface_tickrate = requested_tickrate();

    State {
        size: interface_size,
        tickrate: interface_tickrate,
        interface: PlatformInterface::new(
            interface_size.0 as usize,
            interface_size.1 as usize,
            rng,
        ),
        time: Instant::now(),
        ticks_executed: 0.0,
        accum_time: 0.0,
    }
}

struct TextureInfo {
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    context: G2dTextureContext,
    texture: G2dTexture,
}

fn create_texture_info(window: &mut PistonWindow, state: &State) -> TextureInfo {
    let mut context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let buffer = image::ImageBuffer::new(state.interface.width as u32, state.interface.height as u32);
    let texture: G2dTexture = Texture::from_image(
        &mut context,
        &buffer,
        &TextureSettings::new().filter(Filter::Nearest),
	).unwrap();

    TextureInfo {
        buffer,
        context,
        texture,
    }
}
