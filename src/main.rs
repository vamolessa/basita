extern crate basita;

use basita::sdl::SdlContext;
use basita::systems::SystemCollection;

mod game;
use game::{GameEvents, GameState};
use game::PlayerSystem;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let state = GameState::new(&mut sdl_context);
	let events = GameEvents::new();
	let mut systems = SystemCollection::new();

	systems.add_default_systems();
	systems.add_system::<PlayerSystem>();

	basita::play(state, events, systems);
}
