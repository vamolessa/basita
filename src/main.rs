//extern crate basita;
mod basita;
use basita::*;
use basita::systems::*;

mod game;
use game::*;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let state = GameState::new(&mut sdl_context);
	let events = GameEvents::new();
	let mut systems = SystemCollection::new();

	systems.add_default_systems();
	systems.add_system::<PlayerSystem>();

	play(state, events, systems);
}
