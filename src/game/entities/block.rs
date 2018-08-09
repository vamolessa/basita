use basita::sdl::{SdlLoader, SdlStorage};
use basita::specs::Builder;
use basita::specs::World;

use basita::core::components::Transform;
use basita::math::Vector2;
use basita::physics::components::Collider;
use basita::renderer::components::Sprite;
use basita::renderer::resources::Images;

pub fn new<'a, 'b>(
	world: &mut World,
	sdl_loader: &'a SdlLoader,
	sdl_storage: &'b mut SdlStorage<'a>,
	position: Vector2,
) {
	let image_handle;
	{
		let mut images = world.write_resource::<Images>();

		image_handle = images.load(
			&String::from("assets/images/block.png"),
			&sdl_loader.texture_loader,
			&mut sdl_storage.texture_storage,
		);

		let _image = images.get(image_handle);
	}
	let _block = world
		.create_entity()
		.with(Transform { position: position })
		.with(Sprite {
			image: image_handle,
			..Default::default()
		})
		.with(Collider::new_box(Vector2::new(16.0, 16.0)))
		//.with(::basita::physics::components::PhysicBody::with_inverted_mass(0.0))
		.build();
}
