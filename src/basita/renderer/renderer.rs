use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl::{SdlContext, SdlStorage};
use specs::World;

use super::resources::RenderCommands;

pub fn render<'a>(world: &World, sdl_context: &mut SdlContext, sdl_storage: &SdlStorage<'a>) {
	let clear_color = Color::RGB(0, 0, 0);

	let canvas = &mut sdl_context.canvas;
	let textures = &sdl_storage.texture_storage;

	canvas.set_draw_color(clear_color);
	canvas.clear();

	let commands = world.read_resource::<RenderCommands>();

	for command in commands.iter() {
		let texture = textures.at(command.texture_index);
		let texture_query = texture.query();

		let rect = Rect::new(
			command.position.x,
			command.position.y,
			texture_query.width,
			texture_query.height,
		);

		canvas.copy(texture, None, rect).unwrap();
	}

	canvas.present();
}
