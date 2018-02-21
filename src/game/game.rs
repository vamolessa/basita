use basita::*;

use super::player_system::PlayerSystemData;

pub struct GameState<'a> {
	pub player_system_data: PlayerSystemData,

	pub engine_state: EngineState<'a>,
}

impl<'a> GameState<'a> {
	pub fn new(sdl_context: &'a SdlContext) -> Self {
		let mut engine_state = EngineState::new(sdl_context);

		GameState {
			player_system_data: PlayerSystemData::new(&mut engine_state),
			engine_state: engine_state,
		}
	}
}

impl<'a> ContainsEngineState<'a> for GameState<'a> {
	fn get_engine_state_mut(&mut self) -> &mut EngineState<'a> {
		&mut self.engine_state
	}
}

pub struct GameEvents<'a> {
	pub engine_events: EngineEvents<GameState<'a>, GameEvents<'a>>,
}

impl<'a> GameEvents<'a> {
	pub fn new() -> Self {
		GameEvents {
			engine_events: EngineEvents::new(),
		}
	}
}

impl<'a> ContainsEngineEvents<GameState<'a>, GameEvents<'a>> for GameEvents<'a> {
	fn get_engine_events(&self) -> &EngineEvents<GameState<'a>, GameEvents<'a>> {
		&self.engine_events
	}

	fn get_engine_events_mut(&mut self) -> &mut EngineEvents<GameState<'a>, GameEvents<'a>> {
		&mut self.engine_events
	}
}
