use sdl::SdlContext;
use input::Input;

pub struct EngineState<'a> {
	pub frames_per_second: u32,
	pub delta_time: f32,
	pub running: bool,
	pub sdl_context: &'a SdlContext,
	pub input: Input,
}

impl<'a> EngineState<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		EngineState {
			frames_per_second: 60,
			delta_time: 0.0,
			running: true,
			sdl_context: sdl_context,
			input: Input::new(),
		}
	}
}

/*
pub fn play<'a, S, E>(mut state: S, mut events: E, systems: SystemCollection<S, E>)
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	systems.init(&mut state, &mut events);

	while state.get_engine_state_mut().running {
		systems.update(&mut state, &events);
	}
}
*/
