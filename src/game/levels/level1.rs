use basita::sdl::{SdlLoader, SdlStorage};
use basita::specs::World;

use basita::math::Vector2;

use entities::{block, player};

pub fn load<'a, 'b>(
	world: &mut World,
	sdl_loader: &'a SdlLoader,
	sdl_storage: &'b mut SdlStorage<'a>,
) {
	player::new(world, sdl_loader, sdl_storage, Vector2::new(100.0, 100.0));

	block::new(world, sdl_loader, sdl_storage, Vector2::new(64.0, 64.0));
}
