//extern crate basita;
mod basita;
use basita::*;

mod game;
use game::*;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut engine = Engine::new(&mut sdl_context);

	engine.systems.add(PlayerSystem::new(&mut engine.state));
	engine.systems.add_defaults();

	engine.play();
}
