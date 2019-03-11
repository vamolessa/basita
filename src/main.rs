use basita::{
	game::*,
	mixer::resources::Sfxs,
	renderer::resources::{Fonts, Images},
};

mod game;
use crate::game::{
	entities::{block::*, camera::*, player::*},
	levels::*,
	resources::{GameFonts, GameSfxs},
};

use sdl2::pixels::Color;
use sdl2::rect::Point;

#[derive(Default)]
pub struct MyGame {
	pub images: Images,
	pub fonts: Fonts,
	pub sfxs: Sfxs,

	pub game_fonts: GameFonts,
	pub game_sfxs: GameSfxs,

	pub cameras: Vec<CameraEntity>,
	pub players: Vec<PlayerEntity>,
	pub blocks: Vec<BlockEntity>,
}

impl Game for MyGame {
	fn create(_context: &mut GameContext) -> Self {
		MyGame::default()
	}

	fn update(&mut self, resources: &mut GameResources<MyGame>) {
		resources.renderer.render_commands.add_rect_fill(
			0,
			Color::RGB(255, 0, 0),
			Point::new(100, 100),
			200,
			150,
		);
	}
}

pub fn main() {
	MyGame::play();
}
