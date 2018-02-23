use std::time::Duration;
use std::thread;

use sdl2::pixels::Color;

use super::super::{GameEvents, GameState};
use super::System;

pub struct SdlPresenterSystem;

impl<'a, S, E> System<S, E> for SdlPresenterSystem
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn update(s: &mut S, _e: &E) {
		let state = s.get_engine_state_mut();

		let clear_color = Color::RGB(0, 0, 0);

		let mut canvas = state.sdl_context.canvas.borrow_mut();
		canvas.present();
		thread::sleep(Duration::new(
			0,
			1_000_000_000u32 / state.sdl_context.frames_per_second,
		));

		canvas.set_draw_color(clear_color);
		canvas.clear();

		state.delta_time = 1.0 / (state.sdl_context.frames_per_second as f32);
	}
}
