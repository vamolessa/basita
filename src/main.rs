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

pub fn main() {
	let frames_per_second = 60;
	let clear_color = Color::RGB(0, 0, 0);

	let sdl_context = SdlContext::new("game", 400, 300);
	let mut sdl_storage = SdlStorage::default();

	let mut world = World::new();

	world.register::<Transform>();
	world.register::<Sprite>();

	world.add_resource(Time::default());

	world.add_resource(ImageCollection::default());
	world.add_resource(DirtySprites::default());

	let player_image;

	{
		let mut images = world.write_resource::<ImageCollection>();

		player_image = images.load(
			&String::from("assets/images/player.png"),
			&sdl_context.texture_loader,
			&mut sdl_storage.texture_storage,
		);
	}

	{
		let render_system = RenderSystem::new(&sdl_context, &sdl_storage);
	}

	let mut dispatcher = DispatcherBuilder::new()
		//.add_thread_local(RenderSystem::new(&sdl_context, &sdl_storage))
		.build();

	let _player = world
		.create_entity()
		.with(Transform {
			position: Vector2::new(100.0, 100.0),
		})
		.with(Sprite {
			depth: 0,
			image: player_image,
			renderable_index: 0,
		})
		.build();

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
}
