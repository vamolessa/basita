use basita::sdl::{SdlLoader, SdlStorage};
use basita::specs::World;

use basita::math::Vector2;

use entities::{block, player};

pub fn load<'a, 'b>(
	world: &mut World,
	sdl_loader: &'a SdlLoader,
	sdl_storage: &'b mut SdlStorage<'a>,
) {
	player::new(world, sdl_loader, sdl_storage, Vector2::new(80.0, 100.0));

	block::new(world, sdl_loader, sdl_storage, Vector2::new(200.0, 110.0));

	let hw = 16.0;
	let x0 = hw;
	let y0 = 250.0 - hw;
	for i in 0..(400 / hw as i32) {
		block::new(
			world,
			sdl_loader,
			sdl_storage,
			Vector2::new(x0 + i as f32 * hw * 2.0, y0),
		);
	}

	let hw = 16.0;
	let x0 = hw;
	let y0 = hw;
	for i in 0..(300 / hw as i32) {
		block::new(
			world,
			sdl_loader,
			sdl_storage,
			Vector2::new(x0, y0 + i as f32 * hw * 2.0),
		);
	}
}
