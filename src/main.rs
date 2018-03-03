extern crate basita;

use basita::sdl::SdlContext;
//use basita::systems::SystemCollection;

mod game;
use game::{GameEvents, GameState};
//use game::PlayerSystem;

#[derive(Default)]
struct A
{
	pub a: usize,
	pub b: usize,
}

trait B {
	fn ab(&mut self) -> (&mut usize, &mut usize);
	fn aa(&mut self) -> &mut usize;
	fn bb(&mut self) -> &mut usize;
}

impl B for A {
	fn ab(&mut self) -> (&mut usize, &mut usize) {
		let &mut A {ref mut a, ref mut b} = self;
		(a,b)
	}

	fn aa(&mut self) -> &mut usize {
		let &mut A {ref mut a, ..} = self;
		a
	}

	fn bb(&mut self) -> &mut usize {
		let &mut A {ref mut b, ..} = self;
		b
	}
}

pub fn main() {
	let mut x = A::default();
	let _m = &mut x.a;
	let _n = &mut x.b;

	let mut y = A::default();
	let (_m, _n) = y.ab();

	//let mut sdl_context = SdlContext::new("platform maker", 400, 300);
	//let state = GameState::new(&mut sdl_context);
	//let events = GameEvents::new();
	//let mut systems = SystemCollection::new();

	//systems.add_default_systems();
	//systems.add_system::<PlayerSystem>();

	//basita::play(state, events, systems);
}
