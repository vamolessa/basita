use basita::*;

use super::player_system::PlayerSystemData;

pub struct GameState<'a> {
	pub engine_state: EngineState<'a>,

	pub player_system_data: PlayerSystemData,
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

pub struct GameEvents {
	pub engine_events: EngineEvents,
}

impl GameEvents {
	pub fn new() -> Self {
		GameEvents {
			engine_events: EngineEvents::new(),
		}
	}
}

impl ContainsEngineEvents for GameEvents {
	fn get_engine_events(&self) -> &EngineEvents {
		&self.engine_events
	}

	fn get_engine_events_mut(&mut self) -> &mut EngineEvents {
		&mut self.engine_events
	}
}
