use sdl2::event::Event;

use super::super::{ContainsEngineEvents, ContainsEngineState};
use super::System;

pub struct SdlEventSystem;

impl<'a, S, E> System<S,E> for SdlEventSystem
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	fn update(s: &mut S, _e: &E) {
		let state = s.get_engine_state_mut();

		state.input.update();

		let mut event_pump = state.sdl_context.event_pump.borrow_mut();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => {
					state.running = false;
					break;
				}
				_ => state.input.handle_event(event),
			};
		}
	}
}
