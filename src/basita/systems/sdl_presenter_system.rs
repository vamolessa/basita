use std::time::Duration;

use sdl2::pixels::Color;

use super::super::{EngineEvents, EngineState};
use super::System;

pub struct SdlPresenterSystem {}

impl System for SdlPresenterSystem {
	fn update(state: &mut EngineState, events: &EngineEvents) {
		let clear_color = Color::RGB(0, 0, 0);

		let mut canvas = state.sdl_context.canvas.borrow_mut();
		canvas.present();
		::std::thread::sleep(Duration::new(
			0,
			1_000_000_000u32 / state.sdl_context.frames_per_second,
		));

		canvas.set_draw_color(clear_color);
		canvas.clear();
	}
}
