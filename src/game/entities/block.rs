use basita::sdl2::pixels::Color;
use basita::specs::Builder;

use basita::core::components::Transform;
use basita::core::resources::LazyEvaluations;
use basita::math::Vector2;
use basita::physics::components::Collider;
use basita::renderer::components::Sprite;
use basita::renderer::resources::Images;

pub fn new<'a, 'b>(lazy: &mut LazyEvaluations, position: Vector2) {
	lazy.add(move |world, sdl_loader, sdl_storage| {
		let image_handle;
		{
			let mut images = world.write_resource::<Images>();

			image_handle = images.load(
				&String::from("assets/images/block.png"),
				sdl_loader,
				sdl_storage,
			);
		}

		let _block = world
			.create_entity()
			.with(Transform { position: position })
			.with(Sprite {
				color: Color::RGB(128, 128, 200),
				image: image_handle,
				..Default::default()
			})
			.with(Collider::new_box(Vector2::new(16.0, 16.0)))
			//.with(::basita::physics::components::PhysicBody::with_inverted_mass(0.0))
			.build();
	});
}
