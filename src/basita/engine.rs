use sdl::SdlContext;
use input::Input;

use assets::*;
use components::*;
//use systems::*;
use events::*;

pub trait GameState<'a> {
	fn get_engine_state(&self) -> &EngineState<'a>;
	fn get_engine_state_mut(&mut self) -> &mut EngineState<'a>;
}

pub trait GameEvents<S, E>
where
	E: GameEvents<S, E>,
{
	fn get_engine_events(&self) -> &EngineEvents<S, E>;
	fn get_engine_events_mut(&mut self) -> &mut EngineEvents<S, E>;
}

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

pub struct WorldEvents<S, E>
where
	E: GameEvents<S, E>,
{
	pub on_deserialize: Event<S, E, ()>,
	pub on_load: Event<S, E, ()>,
}

impl<'a, S, E> Default for WorldEvents<S, E>
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn default() -> Self {
		WorldEvents {
			on_deserialize: Event::default(),
			on_load: Event::default(),
		}
	}
}

pub struct EngineEvents<S, E>
where
	E: GameEvents<S, E>,
{
	pub world: WorldEvents<S, E>,
	//pub collision: CollisionEvents<S, E>,
}

impl<'a, S, E> Default for EngineEvents<S, E>
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn default() -> Self {
		EngineEvents {
			world: WorldEvents::default(),
			//collision: CollisionEvents::default(),
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
