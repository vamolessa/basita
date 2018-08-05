use sdl2::pixels::Color;
use sdl2::rect::Point;

use specs::{Join, ReadStorage, System, Write};

use super::super::components::{Collider, Shape};
use core::components::Transform;
use renderer::resources::{RenderCommand, RenderCommands, RenderVariant};

pub struct ColliderRenderSystem;

impl<'s> System<'s> for ColliderRenderSystem {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Collider>,
		Write<'s, RenderCommands>,
	);

	fn run(&mut self, (transforms, colliders, mut commands): Self::SystemData) {
		for (transform, collider) in (&transforms, &colliders).join() {
			let position = transform.position + collider.offset;

			match collider.shape {
				Shape::Box(box_shape) => {
					commands.push(RenderCommand {
						layer: 999,
						position: Point::new(
							(position.x + collider.offset.x - box_shape.half_size.x) as i32,
							(position.y + collider.offset.y - box_shape.half_size.y) as i32,
						),
						render_variant: RenderVariant::Rect(
							Color::RGB(0, 255, 0),
							(box_shape.half_size.x as u32) * 2,
							(box_shape.half_size.y as u32) * 2,
						),
					});
				}
			}
		}
	}
}
