use basita::{self, *};

//use super::player_system::PlayerSystemData;

pub struct GameState<'a> {
	//pub player_system_data: PlayerSystemData,
	pub engine: EngineState<'a>,
	pub world: GameWorld,
}

impl<'a> GameState<'a> {
	pub fn new(sdl_context: &'a sdl::SdlContext) -> Self {
		let mut engine_state = EngineState::new(sdl_context);

		GameState {
			//player_system_data: PlayerSystemData::new(&mut engine_state),
			engine: engine_state,
			world: GameWorld::default(),
		}
	}
}

#[derive(Default)]
pub struct GameWorld {}
