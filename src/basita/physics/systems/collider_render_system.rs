use sdl2::pixels::Color;
use sdl2::rect::Point;

use specs::{Join, ReadStorage, System, Write};

use super::super::components::{Collider, Shape};
use crate::core::components::Transform;
use crate::renderer::components::Camera;
use crate::renderer::resources::RenderCommands;

pub struct ColliderRenderSystem;

impl<'s> System<'s> for ColliderRenderSystem {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Camera>,
		ReadStorage<'s, Collider>,
		Write<'s, RenderCommands>,
	);

	fn run(&mut self, (transforms, cameras, colliders, mut render_commands): Self::SystemData) {
		for camera in cameras.join() {
			for (transform, collider) in (&transforms, &colliders).join() {
				let position = transform.position + collider.offset - camera.position;

				match collider.shape {
					Shape::Box(box_shape) => {
						render_commands.add_rect(
							999,
							Color::RGB(0, 255, 0),
							Point::new(
								(position.x + collider.offset.x - box_shape.half_size.x) as i32,
								(position.y + collider.offset.y - box_shape.half_size.y) as i32,
							),
							(box_shape.half_size.x as u32) * 2,
							(box_shape.half_size.y as u32) * 2,
						);
					}
				}
			}
		}
	}
}
