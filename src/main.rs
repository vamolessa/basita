use std::time::Instant;

use basita::sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};

use basita::{
	core::{
		components::Transform,
		resources::{LazyEvaluations, Time},
	},
	game::*,
	//gui::Gui,
	input::Input,
	//math::Vector2,
	mixer::{resources::Sfxs, Mixer},
	physics::components::{Collider, Shape},
	renderer::{
		components::{Camera, Sprite},
		resources::{Fonts, Images, RenderCommands},
		Renderer,
	},
};

mod game;
use crate::game::{
	entities::{block::*, camera::*, player::*},
	levels::*,
	resources::{GameFonts, GameSfxs},
};

#[derive(Default)]
pub struct MyGame {
	pub time: Time,
	pub input: Input,
	pub renderer: Renderer,
	pub mixer: Mixer,
	pub lazy_evaluations: LazyEvaluations<Self>,

	pub images: Images,
	pub fonts: Fonts,
	pub sfxs: Sfxs,
	//pub gui: Gui,
	pub game_fonts: GameFonts,
	pub game_sfxs: GameSfxs,

	pub cameras: Vec<CameraEntity>,
	pub players: Vec<PlayerEntity>,
	pub blocks: Vec<BlockEntity>,
}

impl MyGame {
	fn update(&mut self) {
		let move_velocity = 60.0;
		let jump_impulse = 100.0;

		for e in &mut self.players {
			e.physic_body.velocity.x = 0.0;
			if self.input.key(Keycode::Left).is_pressed {
				e.physic_body.velocity.x -= move_velocity;
			}
			if self.input.key(Keycode::Right).is_pressed {
				e.physic_body.velocity.x += move_velocity;
			}

			if self.input.key(Keycode::Up).just_pressed() {
				e.physic_body.velocity.y -= jump_impulse;
			}

			e.physic_body.acceleration.y = 100.0;
		}
	}

	fn draw(&mut self) {
		for camera in &self.cameras {
			for e in &self.players {
				draw_sprite(
					&mut self.renderer.render_commands,
					&self.images,
					&camera.camera,
					&e.transform,
					&e.sprite,
				);

				draw_collider(
					&mut self.renderer.render_commands,
					&camera.camera,
					&e.transform,
					&e.collider,
				)
			}

			for e in &self.blocks {
				draw_sprite(
					&mut self.renderer.render_commands,
					&self.images,
					&camera.camera,
					&e.transform,
					&e.sprite,
				);

				draw_collider(
					&mut self.renderer.render_commands,
					&camera.camera,
					&e.transform,
					&e.collider,
				)
			}
		}

		/*
		self.gui.font_handle = self.game_fonts.consola_32;
		self.gui.layer = 100;
		self.gui.color = Color::RGB(255, 100, 100);
		self.gui
			.label(Point::new(10, 10), "Coe, lek", Vector2::zero());
		*/
	}
}

impl Game for MyGame {
	fn create(_context: &mut GameContext) -> Self {
		let mut game = MyGame::default();
		level1::load(&mut game.lazy_evaluations);
		game
	}

	fn run(&mut self, context: &mut GameContext) {
		'main: loop {
			let frame_start_instant = Instant::now();

			self.input.update();
			let event_pump = &mut context.sdl_context.event_pump;
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. } => {
						break 'main;
					}
					e => {
						self.input.handle_event(e);
					}
				};
			}

			self.update();
			self.draw();

			LazyEvaluations::evaluate(context, self, |g| &mut g.lazy_evaluations);
			self.renderer
				.render(&mut context.sdl_context, &mut context.sdl_storage)
				.unwrap();
			self.mixer.mix(&mut context.sdl_storage).unwrap();

			self.time
				.sleep_rest_of_frame(context.settings.frames_per_second, &frame_start_instant);
		}
	}
}

fn draw_sprite(
	render_commands: &mut RenderCommands,
	images: &Images,
	camera: &Camera,
	transform: &Transform,
	sprite: &Sprite,
) {
	let image = images.get(sprite.image);
	let position = transform.position - camera.position;

	render_commands.add_texture_ex(
		sprite.layer,
		sprite.color,
		Point::new(position.x as i32, position.y as i32) - image.center,
		image.texture_index,
		sprite.flip_horizontal,
		sprite.flip_vertical,
	);
}

fn draw_collider(
	render_commands: &mut RenderCommands,
	camera: &Camera,
	transform: &Transform,
	collider: &Collider,
) {
	let position = transform.position + collider.offset - camera.position;

	match collider.shape {
		Shape::Box(box_shape) => {
			render_commands.add_rect(
				999,
				Color::RGB(0, 255, 0),
				Point::new(
					(position.x + collider.offset.x - box_shape.half_size.x) as i32,
					(position.y + collider.offset.y - box_shape.half_size.y) as i32,
				),
				(box_shape.half_size.x as u32) * 2,
				(box_shape.half_size.y as u32) * 2,
			);
		}
	}
}

pub fn main() {
	MyGame::play(GameSettings {
		frames_per_second: 60,
		..GameSettings::default()
	});
}
