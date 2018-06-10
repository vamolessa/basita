use basita::math::Vector2;
use basita::sdl::{SdlContext, SdlStorage};
use basita::specs::World;

use entities::{block, player};

pub fn load<'a, 'b>(
	world: &mut World,
	sdl_context: &'a SdlContext,
	sdl_storage: &'b SdlStorage<'a>,
) {
	player::new(world, sdl_context, sdl_storage, Vector2::new(100.0, 100.0));

	block::new(world, sdl_context, sdl_storage, Vector2::new(0.0, 0.0));
}
