extern crate basita;

use std::time::Duration;
use std::thread;

use basita::sdl2::pixels::Color;
use basita::sdl2::event::Event;
use basita::specs::{DispatcherBuilder, World};

use basita::sdl::SdlContext;
use basita::renderer::systems::RenderSystem;

use basita::core::components::Transform;
use basita::core::resources::Time;

use basita::renderer::components::Sprite;
use basita::renderer::resources::{UpdatedSprites, ImageCollection};

pub fn main() {
	let frames_per_second = 60;
	let clear_color = Color::RGB(0, 0, 0);

	let sdl_context = SdlContext::new("platform maker", 400, 300);
	let mut world = World::new();

	let mut dispatcher = DispatcherBuilder::new()
		.add_thread_local(RenderSystem::new(&sdl_context))
		.build();

	world.register::<Transform>();
	world.register::<Sprite>();

	world.add_resource(Time::default());
	world.add_resource(UpdatedSprites::default());
	world.add_resource(ImageCollection::default());

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
