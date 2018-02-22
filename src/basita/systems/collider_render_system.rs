use sdl2::rect::Rect;
use sdl2::pixels::Color;

use super::System;
use super::super::{ContainsEngineEvents, ContainsEngineState};
use super::super::components::Shape;

pub struct ColliderRenderSystem;

impl<'a, S, E> System<S, E> for ColliderRenderSystem
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents<S, E>,
{
	fn update(s: &mut S, _e: &E) {
		let state = s.get_engine_state_mut();

		let mut canvas = state.sdl_context.canvas.borrow_mut();

		for collider in state.colliders.iter() {
			canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));

			let transform = state.transforms.get(collider.transform);
			let position = transform.position + collider.offset;

			match collider.shape {
				Shape::Box(box_shape) => {
					let half_size = box_shape.half_size;

					canvas
						.draw_rect(Rect::new(
							(position.x - half_size.x) as i32,
							(position.y - half_size.y) as i32,
							(half_size.x as u32) * 2,
							(half_size.y as u32) * 2,
						))
						.unwrap();
				}
			}
		}
	}
}
