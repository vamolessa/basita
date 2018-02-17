//extern crate basita;
mod basita;
use basita::*;

mod game;
use game::*;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut state = EngineState::new(&mut sdl_context);
	let mut systems = EngineSystems::new();

	systems.add_defaults();
	systems.add(PlayerSystem::new(&mut state));

	play(&mut state, &systems);
}
