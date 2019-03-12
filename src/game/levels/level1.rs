use basita::{core::resources::LazyEvaluations, math::Vector2};

use crate::{
	game::entities::{block, camera, player},
	MyGame,
};

pub fn load(lazy: &mut LazyEvaluations<MyGame>) {
	lazy.add(|context, game| {
		{
			game.game_fonts.consola_32 = game.fonts.load(
				&(String::from("assets/fonts/consola.ttf"), 32),
				context.sdl_loader,
				&mut context.sdl_storage,
			);
		}

		{
			game.game_sfxs.hit = game.sfxs.load(
				&String::from("assets/audios/hit.wav"),
				context.sdl_loader,
				&mut context.sdl_storage,
			);
		}

		camera::new(&mut game.lazy_evaluations, Vector2::new(0.0, 0.0));
		player::new(&mut game.lazy_evaluations, Vector2::new(80.0, 100.0));
		block::new(&mut game.lazy_evaluations, Vector2::new(200.0, 110.0));

		let hw = 16.0;
		let x0 = hw;
		let y0 = 256.0 - hw;
		for i in 0..(400 / hw as i32) {
			block::new(
				&mut game.lazy_evaluations,
				Vector2::new(x0 + i as f32 * hw * 2.0, y0),
			);
		}

		let hw = 16.0;
		let x0 = hw;
		let y0 = hw;
		for i in 0..(300 / hw as i32) {
			block::new(
				&mut game.lazy_evaluations,
				Vector2::new(x0, y0 + i as f32 * hw * 2.0),
			);
		}
	});
}
