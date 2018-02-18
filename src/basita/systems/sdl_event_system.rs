use sdl2::event::Event;

use super::super::{ContainsEngineEvents, ContainsEngineState};

pub fn update<'a, S, E>(s: &mut S, _e: &E)
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents,
{
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
