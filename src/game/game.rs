use basita::*;

use super::player_system::PlayerSystemData;

pub struct GameState {
	pub player_system_data: PlayerSystemData,
}

impl GameState {
	pub fn new() -> Self {
		let mut engine_state = EngineState::new();

		GameState {
			player_system_data: PlayerSystemData::new(&mut engine_state),
			engine_state: engine_state,
		}
	}
}

pub struct GameEvents {
}

impl GameEvents {
	pub fn new() -> Self {
		GameEvents {
			engine_events: EngineEvents::new(),
		}
	}
}
