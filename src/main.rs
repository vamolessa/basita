//extern crate basita;
mod basita;
use basita::*;
use basita::systems::*;

mod game;
use game::*;

/*
TODO
SystemCollection: Vec<Box<System>> => Vec<Rc<System>>
SystemCollection recebe as traits que os systems implementam
Ta na hora de fazer um collection/handle generico?? (acho que ainda nao)

volta com EngineSystems (que tem um SystemCollection)
Event tem um SystemCollection
*/

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut state = EngineState::new(&mut sdl_context);
	let systems = EngineSystems::new();

	let player_system = PlayerSystem::new(&mut state);

	systems.init(&mut state);
	player_system.init(&mut state);

	while state.running {
		systems.update(&mut state);
		player_system.update(&mut state);
	}
}
