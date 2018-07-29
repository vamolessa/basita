use basita::core::components::Transform;
use basita::creator::Creator;
use basita::math::Vector2;
use basita::physics::components::new_box_collider;
use basita::renderer::components::Sprite;
use basita::renderer::resources::Images;

pub fn new<'a: 'b, 'b>(creator: &mut Creator<'a, 'b>, position: Vector2) {
	let image_handle;
	{
		let mut images = creator.world.write_resource::<Images>();

		image_handle = images.load(
			&String::from("assets/images/block.png"),
			&creator.sdl_context.texture_loader,
			&mut creator.sdl_storage.texture_storage.borrow_mut(),
		);

		let _image = images.get(image_handle);
	}

	let _block = creator
		.world
		.create_entity()
		.with(Transform { position: position })
		.with(Sprite {
			layer_index: 0,
			image: image_handle,
		})
		.with(new_box_collider(Vector2::new(16.0, 16.0)))
		.build();
}
