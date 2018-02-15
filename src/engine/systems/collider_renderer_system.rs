use sdl2::rect::Rect;
use sdl2::pixels::Color;

use sdl_context::SdlContext;
use components::{BoxCollider, ComponentCollection};

pub struct ColliderRendererSystem {}

impl<'a> ColliderRendererSystem {
	pub fn update(
		&self,
		sdl: &mut SdlContext,
		box_collider_collection: &ComponentCollection<BoxCollider>,
	) {
		for box_collider in &box_collider_collection.all {
			let transform = super::super::components::Transform {
				position: super::super::math::Vector2::new(0.0, 0.0),
			};

			sdl.canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));
			sdl.canvas
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
