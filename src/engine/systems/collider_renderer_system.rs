use sdl2::rect::Rect;
use sdl2::pixels::Color;

use application::Application;
use components::{BoxCollider, ComponentCollection};

pub struct ColliderRendererSystem {}

impl<'a> ColliderRendererSystem {
	pub fn update(
		&self,
		app: &mut Application,
		box_collider_collection: &ComponentCollection<BoxCollider>,
	) {
		for box_collider in &box_collider_collection.all {
			let transform = box_collider.transform;

			app.canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));
			app.canvas
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
