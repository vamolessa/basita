use basita;
use basita::*;

use super::player_system::PlayerSystemData;

pub struct GameState<'a> {
	pub player_system_data: PlayerSystemData,

	pub engine: EngineState<'a>,
	pub world: GameWorld,
}

impl<'a> GameState<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		let mut engine_state = EngineState::new(sdl_context);

		GameState {
			player_system_data: PlayerSystemData::new(&mut engine_state),
			engine: engine_state,
			world: GameWorld::default(),
		}
	}
}

#[derive(Default)]
pub struct GameWorld {}

impl<'a> basita::GameState<'a> for GameState<'a> {
	fn get_engine_state(&self) -> &EngineState<'a> {
		&self.engine
	}

	fn get_engine_state_mut(&mut self) -> &mut EngineState<'a> {
		&mut self.engine
	}
}

pub struct GameEvents<'a> {
	pub events: EngineEvents<GameState<'a>, GameEvents<'a>>,
}

impl<'a> GameEvents<'a> {
	pub fn new() -> Self {
		GameEvents {
			events: EngineEvents::default(),
		}
	}
}

impl<'a> basita::GameEvents<GameState<'a>, GameEvents<'a>> for GameEvents<'a> {
	fn get_engine_events(&self) -> &EngineEvents<GameState<'a>, GameEvents<'a>> {
		&self.events
	}

	fn get_engine_events_mut(&mut self) -> &mut EngineEvents<GameState<'a>, GameEvents<'a>> {
		&mut self.events
	}
}
