use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl::{SdlContext, SdlStorage};
use specs::World;

use super::resources::{RenderCommands, RenderVariant};

pub fn render<'a>(
	world: &mut World,
	sdl_context: &mut SdlContext,
	sdl_storage: &mut SdlStorage<'a>,
) {
	let canvas = &mut sdl_context.canvas;

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();

	let mut commands = world.write_resource::<RenderCommands>();
	commands.sort_by_key(|c| c.layer);

	for command in commands.iter() {
		match command.variant {
			RenderVariant::Texture(texture_index) => {
				let texture = sdl_storage.texture_storage.at_mut(texture_index);
				let texture_query = texture.query();

				texture.set_color_mod(command.color.r, command.color.g, command.color.b);
				texture.set_alpha_mod(command.color.a);

				let rect = Rect::new(
					command.position.x,
					command.position.y,
					texture_query.width,
					texture_query.height,
				);

				canvas.copy(texture, None, rect).unwrap();
			}
			RenderVariant::TextureEx(texture_index, flip_horizontal, flip_vertical) => {
				let texture = sdl_storage.texture_storage.at_mut(texture_index);
				let texture_query = texture.query();

				texture.set_color_mod(command.color.r, command.color.g, command.color.b);
				texture.set_alpha_mod(command.color.a);

				let rect = Rect::new(
					command.position.x,
					command.position.y,
					texture_query.width,
					texture_query.height,
				);

				canvas
					.copy_ex(
						texture,
						None,
						rect,
						0.0,
						None,
						flip_horizontal,
						flip_vertical,
					)
					.unwrap();
			}
			RenderVariant::Rect(width, height) => {
				canvas.set_draw_color(command.color);
				canvas
					.draw_rect(Rect::new(
						command.position.x,
						command.position.y,
						width,
						height,
					))
					.unwrap();
			}
		}
	}

	canvas.present();
}
