use basita::game::*;

mod game;
use crate::game::*;

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

use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct MyGame {}

impl Game for MyGame {
	fn create(_context: &mut GameContext) -> Self {
		MyGame {}
	}

	fn update(&mut self, resources: &mut GameResources<MyGame>) {
		resources.renderer.render_commands.add_rect_fill(
			0,
			Color::RGB(255, 0, 0),
			Point::new(100, 100),
			200,
			150,
		);
	}
}

pub fn main() {
	MyGame::play();
}
