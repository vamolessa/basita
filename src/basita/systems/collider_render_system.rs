use sdl2::rect::Rect;
use sdl2::pixels::Color;

use super::System;
use super::super::{GameEvents, GameState};
use super::super::components::Shape;

pub struct ColliderRenderSystem;

impl<'a, S, E> System<S, E> for ColliderRenderSystem
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn update(s: &mut S, _e: &E) {
		let state = s.get_engine_state();
		let mut canvas = state.sdl_context.canvas.borrow_mut();
		let world = &state.world;

		for &(_h, collider) in world.colliders.iter() {
			canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));

			let transform = state.world.transforms.get(&collider.transform);
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
