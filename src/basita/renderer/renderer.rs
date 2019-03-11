use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::sdl::{SdlContext, SdlStorage};

use super::resources::{RenderCommands, RenderVariant};

#[derive(Default)]
pub struct Renderer {
	pub render_commands: RenderCommands,
}

impl Renderer {
	pub fn render<'a>(&mut self, sdl_context: &mut SdlContext, sdl_storage: &mut SdlStorage<'a>) {
		render(&mut self.render_commands, sdl_context, sdl_storage).unwrap();
	}
}

pub fn render<'a>(
	render_commands: &mut RenderCommands,
	sdl_context: &mut SdlContext,
	sdl_storage: &mut SdlStorage<'a>,
) -> Result<(), String> {
	let canvas = &mut sdl_context.canvas;

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();

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

				canvas.copy(texture, None, rect)?;
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

				canvas.copy_ex(
					texture,
					None,
					rect,
					0.0,
					None,
					flip_horizontal,
					flip_vertical,
				)?;
			}
			RenderVariant::Rect(width, height) => {
				canvas.set_draw_color(command.color);
				canvas.draw_rect(Rect::new(
					command.position.x,
					command.position.y,
					width,
					height,
				))?;
			}
			RenderVariant::RectFill(width, height) => {
				canvas.set_draw_color(command.color);
				canvas.fill_rect(Rect::new(
					command.position.x,
					command.position.y,
					width,
					height,
				))?;
			}
			RenderVariant::Line(to_position) => {
				canvas.set_draw_color(command.color);
				canvas.draw_line(command.position, to_position)?;
			}
			RenderVariant::Point => {
				canvas.set_draw_color(command.color);
				canvas.draw_point(command.position)?;
			}
		}
	}

	canvas.present();
	render_commands.commands.clear();

	Ok(())
}
