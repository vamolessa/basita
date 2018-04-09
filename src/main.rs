extern crate basita;

use std::thread;
use std::time::Duration;

use basita::sdl2::event::Event;
use basita::sdl2::pixels::Color;
use basita::specs::{DispatcherBuilder, World};

use basita::renderer::systems::RenderSystem;
use basita::sdl::{SdlContext, SdlStorage};

use basita::core::components::Transform;
use basita::core::resources::Time;
use basita::math::Vector2;

use basita::renderer::components::Sprite;
use basita::renderer::resources::{DirtySprites, ImageCollection};

use basita::core::assets::AssetLoader;
use basita::sdl::{TextureLoader, Textures};
use basita::sdl2::image::LoadTexture;

pub fn main() {
	/*
	let frames_per_second = 60;
	let clear_color = Color::RGB(0, 0, 0);
	*/

	let sdl_context = SdlContext::new("game", 400, 300);
	let mut texture_loader = TextureLoader::new(sdl_context.canvas.borrow().texture_creator());
	let mut sdl_storage = SdlStorage::default();

	(&texture_loader, &mut sdl_storage.texture_storage)
		.load("path")
		.unwrap();

	//let mut tex_colection = Vec::new();

	//let tex = textures.texture_creator.load_texture("path").unwrap();
	//tex_colection.push(tex);

	/*
	let (t, i) = image_loader
		.load_texture(&String::from("assets/images/player.png"))
		.unwrap();
		*/

	/*
	images.load(
		&String::from("assets/images/player.png"),
		image_loader.deref_mut(),
	);
	*/

	//----------------------

	/*
	let mut dispatcher = DispatcherBuilder::new()
		.add_thread_local(RenderSystem::new(&sdl_context))
		.build();

	world.register::<Transform>();
	world.register::<Sprite>();

	world.add_resource(Time::default());

	world.add_resource(ImageCollection::default());
	world.add_resource(DirtySprites::default());

	{
		let player_image;

		{
			let loader = &mut sdl_context.textures;
			let mut images = world.write_resource::<ImageCollection>();

			player_image = images.load(&String::from("assets/images/player.png"), loader);
			images.load(&String::from("assets/images/player.png"), loader);
		}

		let _player = world.create_entity()
			.with(Transform {
				position: Vector2::new(100.0, 100.0),
			})
			.with(Sprite {
				depth: 0,
				image: player_image,
				renderable_index: 0,
			})
			.build();
	}

	'main: loop {
		let mut event_pump = sdl_context.event_pump.borrow_mut();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => {
					break 'main;
				}
				_ => (),
			};
		}

		{
			let mut canvas = sdl_context.canvas.borrow_mut();

			canvas.set_draw_color(clear_color);
			canvas.clear();
		}

		dispatcher.dispatch(&mut world.res);

		{
			let mut canvas = sdl_context.canvas.borrow_mut();
			canvas.present();
		}

		world.maintain();

		thread::sleep(Duration::new(0, 1_000_000_000u32 / frames_per_second));
	}
	*/
}
