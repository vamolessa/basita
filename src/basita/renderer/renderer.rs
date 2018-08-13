use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl::{font_print, SdlContext, SdlStorage};
use specs::World;

use super::resources::{Fonts, RenderCommands, RenderVariant};

pub fn render<'a>(world: &World, sdl_context: &mut SdlContext, sdl_storage: &SdlStorage<'a>) {
	let canvas = &mut sdl_context.canvas;
	let textures = &sdl_storage.texture_storage;

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();

	let commands = world.read_resource::<RenderCommands>();

	for command in commands.iter() {
		match command.render_variant {
			RenderVariant::Texture(texture_index, flip_horizontal, flip_vertical) => {
				let texture = textures.at(texture_index);
				let texture_query = texture.query();

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
			RenderVariant::Rect(color, width, height) => {
				canvas.set_draw_color(color);
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

	use sdl2::rect::Point;

	let fonts = world.read_resource::<Fonts>();
	for font in fonts.asset_iter() {
		font_print(
			canvas,
			sdl_storage,
			Point::new(100, 0),
			&String::from("Hello"),
			&font.glyphs,
		).unwrap();
	}

	canvas.present();
}
