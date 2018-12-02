use sdl2::rect::Point;

use specs::{Join, Read, ReadStorage, System, Write};

use super::components::{Camera, Sprite};
use super::resources::{Images, RenderCommand, RenderCommands, RenderVariant};
use core::components::Transform;

pub struct RenderSystem;

impl<'s> System<'s> for RenderSystem {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Camera>,
		ReadStorage<'s, Sprite>,
		Read<'s, Images>,
		Write<'s, RenderCommands>,
	);

	fn run(
		&mut self,
		(transforms, cameras, sprites, images, mut render_commands): Self::SystemData,
	) {
		for camera in cameras.join() {
			for (transform, sprite) in (&transforms, &sprites).join() {
				let image = images.get(sprite.image);

				let position = transform.position - camera.position;

				render_commands.commands.push(RenderCommand {
					layer: sprite.layer,
					position: Point::new(position.x as i32, position.y as i32) - image.center,
					color: sprite.color,
					variant: RenderVariant::TextureEx(
						image.index,
						sprite.flip_horizontal,
						sprite.flip_vertical,
					),
				});
			}
		}
	}
}
