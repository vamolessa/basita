use sdl2::event::Event;

use super::super::EngineState;
use super::System;

pub struct SdlEventSystem {}

impl System for SdlEventSystem {
	fn update(&mut self, state: &mut EngineState) -> bool {
		let mut event_pump = state.sdl_context.event_pump.borrow_mut();
		for event in event_pump.poll_iter() {
			let should_continue = match event {
				Event::Quit { .. } => false,
				_ => state.input.handle_event(event),
			};

			if !should_continue {
				return false;
			}
		}

		true
	}
}
