use sdl2::rect::Rect;
use sdl2::pixels::Color;

use SdlContext;
use components::{BoxCollider, ComponentCollection};

pub struct ColliderRenderSystem {}

impl<'a> ColliderRenderSystem {
	pub fn update(
		&self,
		sdl: &SdlContext,
		box_collider_collection: &ComponentCollection<BoxCollider>,
	) {
		let mut canvas = sdl.canvas.borrow_mut();

		for box_collider in &box_collider_collection.all {
			let transform = super::super::components::Transform {
				position: super::super::math::Vector2::new(0.0, 0.0),
			};

			canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));
			canvas
				.draw_rect(Rect::new(
					transform.position.x as i32,
					transform.position.y as i32,
					box_collider.size.x as u32,
					box_collider.size.x as u32,
				))
				.unwrap();
		}
	}
}
