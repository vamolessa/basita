use basita::specs::Builder;

use basita::core::components::Transform;
use basita::core::resources::LazyEvaluations;
use basita::math::Vector2;
use basita::physics::components::{Collider, PhysicBody};
use basita::renderer::components::Sprite;
use basita::renderer::resources::Images;

use components::Player;

pub fn new<'a, 'b>(lazy: &mut LazyEvaluations, position: Vector2) {
	lazy.add(move |world, sdl_loader, sdl_storage| {
		let image_handle;
		{
			let mut images = world.write_resource::<Images>();

			image_handle = images.load(
				&String::from("assets/images/player.png"),
				sdl_loader,
				sdl_storage,
			);
		}

		let _player = world
			.create_entity()
			.with(Player)
			.with(Transform { position: position })
			.with(Sprite {
				image: image_handle,
				..Default::default()
			})
			.with(Collider::new_box(Vector2::new(16.0, 16.0)))
			.with(PhysicBody::with_inverted_mass(1.0))
			.build();
	});
}
