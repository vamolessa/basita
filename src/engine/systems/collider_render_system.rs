use sdl2::rect::Rect;
use sdl2::pixels::Color;

use super::super::EngineState;
use super::System;

pub struct ColliderRenderSystem {}

impl System for ColliderRenderSystem {
	fn update(&mut self, state: &mut EngineState) {
		let mut canvas = state.sdl_context.canvas.borrow_mut();

		for box_collider in &state.box_colliders.all {
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
