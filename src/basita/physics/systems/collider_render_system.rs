use specs::{Join, ReadStorage, System};

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use super::super::components::{Collider, Shape};
use core::components::Transform;
use sdl::SdlContext;

pub struct ColliderRenderSystem<'a> {
	sdl_context: &'a SdlContext<'a>,
}

impl<'a> ColliderRenderSystem<'a> {
	pub fn new(sdl_context: &'a SdlContext<'a>) -> Self {
		ColliderRenderSystem {
			sdl_context: sdl_context,
		}
	}
}

impl<'a, 's> System<'s> for ColliderRenderSystem<'a> {
	type SystemData = (ReadStorage<'s, Transform>, ReadStorage<'s, Collider>);

	fn run(&mut self, (transforms, colliders): Self::SystemData) {
		let mut canvas = self.sdl_context.canvas.borrow_mut();
		canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));

		for (transform, collider) in (&transforms, &colliders).join() {
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
