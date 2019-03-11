use basita::game::*;

//mod game;
//use crate::game::*;

/* Quase funciona uma struct que engloba tudo (sdl_loader precisa ficar de fora)
struct Test<'a> {
	context: SdlContext,
	loader: &'a SdlLoader,
	storage: SdlStorage<'a>,
}

pub fn test() {
	let sdl_context = SdlContext::new("game", 400, 300, 8);
	let sdl_loader = SdlLoader::new(&sdl_context); // drop 1
	let sdl_storage = SdlStorage::default(); // drop 2

	let mut sdl = Test {
		context: sdl_context,
		loader: &sdl_loader,
		storage: sdl_storage,
	};

	{
		let mut world = World::new();
		LazyEvaluations::evaluate(&mut world, &sdl.loader, &mut sdl.storage);
	}
}
*/

pub struct MyGame {}

impl Game for MyGame {
	fn create(_context: &mut GameContext) -> Self {
		MyGame {}
	}
}

pub fn main() {
	MyGame::play();
}
