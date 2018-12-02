use basita::specs::Builder;

use basita::core::resources::LazyEvaluations;
use basita::math::Vector2;
use basita::mixer::resources::Sfxs;
use basita::renderer::components::Camera;
use basita::renderer::resources::Fonts;

use entities::{block, player};
use resources::{GameFonts, GameSfxs};

pub fn load(lazy: &mut LazyEvaluations) {
	lazy.add(move |world, sdl_loader, sdl_storage| {
		{
			let mut game_fonts = world.write_resource::<GameFonts>();
			let mut fonts = world.write_resource::<Fonts>();
			game_fonts.consola_32 = fonts.load(
				&(String::from("assets/fonts/consola.ttf"), 32),
				sdl_loader,
				sdl_storage,
			);
		}

		{
			let mut game_sfxs = world.write_resource::<GameSfxs>();
			let mut sfxs = world.write_resource::<Sfxs>();
			game_sfxs.hit = sfxs.load(
				&String::from("assets/audios/hit.wav"),
				sdl_loader,
				sdl_storage,
			);
		}

		let _camera = world
			.create_entity()
			.with(Camera {
				position: Vector2::new(0.0, 0.0),
			})
			.build();

		let lazy = &mut world.write_resource::<LazyEvaluations>();

		player::new(lazy, Vector2::new(80.0, 100.0));
		block::new(lazy, Vector2::new(200.0, 110.0));

		let hw = 16.0;
		let x0 = hw;
		let y0 = 256.0 - hw;
		for i in 0..(400 / hw as i32) {
			block::new(lazy, Vector2::new(x0 + i as f32 * hw * 2.0, y0));
		}

		let hw = 16.0;
		let x0 = hw;
		let y0 = hw;
		for i in 0..(300 / hw as i32) {
			block::new(lazy, Vector2::new(x0, y0 + i as f32 * hw * 2.0));
		}
	});
}
