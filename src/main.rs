//extern crate basita;
mod basita;
use basita::*;
use basita::systems::*;

mod game;
use game::*;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut state = GameState::new(&mut sdl_context);
	let mut events = GameEvents::new();
	let mut systems = SystemCollection::new();

	systems.add_default_systems();
	systems.add_system(Some(player_system::init), player_system::update);

	systems.init(&mut state, &mut events);

	let message = String::from("ASDasdasd");
	events
		.engine_events
		.some_event
		.raise(&mut state.engine_state, &message);

	while state.engine_state.running {
		systems.update(&mut state, &events);
	}
}
