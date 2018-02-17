//extern crate basita;
mod basita;
use basita::*;
use basita::systems::*;

mod game;
use game::*;

pub fn main() {
	let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut state = EngineState::new(&mut sdl_context);
	let mut events = EngineEvents::new();
	let mut systems = SystemCollection::new();

	systems.add_defaults();
	systems.add::<PlayerSystem>();

	systems.init(&mut state, &mut events);

	let message = String::from("ASDasdasd");
	events.some_event.raise(&mut state, &message);

	while state.running {
		systems.update(&mut state, &events);
	}
}
