use basita::core::components::Transform;
use basita::math::Vector2;
use basita::renderer::components::Sprite;
use basita::renderer::resources::Images;
use basita::sdl::{SdlContext, SdlStorage};
use basita::specs::World;

pub fn new<'a, 'b>(
	world: &mut World,
	sdl_context: &'a SdlContext,
	sdl_storage: &'b SdlStorage<'a>,
	position: Vector2,
) {
	let player_image;
	{
		let mut images = world.write_resource::<Images>();

		player_image = images.load(
			&String::from("assets/images/player.png"),
			&sdl_context.texture_loader,
			&mut sdl_storage.texture_storage.borrow_mut(),
		);

		images.get(player_image);
	}

	let _player = world
		.create_entity()
		.with(Transform { position: position })
		.with(Sprite {
			layer_index: 0,
			image: player_image,
		})
		.build();
}
