use basita::core::components::Transform;
use basita::math::Vector2;
use basita::physics::components::new_box_collider;
use basita::renderer::components::Sprite;
use basita::renderer::resources::Images;
use basita::sdl::{SdlContext, SdlStorage};
use basita::specs::World;

use components::Player;

pub fn new<'a, 'b>(
	world: &mut World,
	sdl_context: &'a SdlContext,
	sdl_storage: &'b SdlStorage<'a>,
	position: Vector2,
) {
	let image_handle;
	{
		let mut images = world.write_resource::<Images>();

		image_handle = images.load(
			&String::from("assets/images/player.png"),
			&sdl_context.texture_loader,
			&mut sdl_storage.texture_storage.borrow_mut(),
		);

		let _image = images.get(image_handle);
	}

	let _player = world
		.create_entity()
		.with(Player)
		.with(Transform { position: position })
		.with(Sprite {
			layer_index: 0,
			image: image_handle,
		})
		.with(new_box_collider(Vector2::new(16.0, 16.0)))
		.build();
}
