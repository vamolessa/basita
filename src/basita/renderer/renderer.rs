use sdl2::pixels::Color;
use sdl2::rect::Rect;

use specs::World;

use crate::sdl::{SdlContext, SdlStorage};

use super::resources::{RenderCommands, RenderVariant};

pub fn render<'a>(
	world: &mut World,
	sdl_context: &mut SdlContext,
	sdl_storage: &mut SdlStorage<'a>,
) {
	let canvas = &mut sdl_context.canvas;

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();

	let mut render_commands = world.write_resource::<RenderCommands>();
	render_commands.commands.sort_by_key(|c| c.layer);

	for command in render_commands.commands.iter() {
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
			RenderVariant::RectFill(width, height) => {
				canvas.set_draw_color(command.color);
				canvas
					.fill_rect(Rect::new(
						command.position.x,
						command.position.y,
						width,
						height,
					))
					.unwrap();
			}
			RenderVariant::Line(to_position) => {
				canvas.set_draw_color(command.color);
				canvas.draw_line(command.position, to_position).unwrap();
			}
			RenderVariant::Point => {
				canvas.set_draw_color(command.color);
				canvas.draw_point(command.position).unwrap();
			}
		}
	}

	canvas.present();
	render_commands.commands.clear();
}
