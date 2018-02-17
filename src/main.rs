use std::rc::Rc;

//extern crate basita;
mod basita;
use basita::*;
use basita::systems::*;

mod game;
use game::*;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut state = EngineState::new(&mut sdl_context);

	let mut systems = SystemCollection::new();
	add_default_systems(&mut systems);
	systems.add(PlayerSystem::new(&mut state));

	play(state, Rc::new(systems));
}
