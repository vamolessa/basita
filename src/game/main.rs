extern crate basita;

use basita::*;
use basita::systems::System;

mod player_system;
use player_system::PlayerSystem;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut engine = Engine::new(&mut sdl_context);

	let mut systems: Vec<Box<System>> = vec![Box::from(PlayerSystem::new(&mut engine.state))];

	engine.systems.add_default_and_custom_systems(&mut systems);
	engine.play();
}
