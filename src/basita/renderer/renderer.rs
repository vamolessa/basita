use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl::{SdlContext, SdlStorage};
use specs::World;

use super::resources::{Images, RenderLayers};

pub fn render<'a>(world: &World, sdl_context: &mut SdlContext, sdl_storage: &SdlStorage<'a>) {
	let clear_color = Color::RGB(0, 0, 0);

	let canvas = &mut sdl_context.canvas;
	let textures = &sdl_storage.texture_storage;

	canvas.set_draw_color(clear_color);
	canvas.clear();

	let layers = world.read_resource::<RenderLayers>();
	let images = world.read_resource::<Images>();

	for layer in layers.iter() {
		for image_instance in layer.iter() {
			let image = images.get(image_instance.image);
			let texture = textures.at(image.texture_index);

			let position = image_instance.position - image.center;
			let rect = Rect::new(position.x, position.y, image.width, image.height);

			canvas.copy(texture, None, rect).unwrap();
		}
	}

	canvas.present();
}
