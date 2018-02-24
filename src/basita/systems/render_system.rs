use sdl2::rect::Rect;

use super::super::{GameEvents, GameState};
use super::super::components::{ComponentHandle, Sprite};
use super::System;

#[derive(Default)]
pub struct RenderSystemState<'a> {
	sprites: Vec<ComponentHandle<Sprite<'a>>>,
}

impl<'a> RenderSystemState<'a> {
	pub fn add_sprite(&mut self, sprite: ComponentHandle<Sprite<'a>>) {
		self.sprites.push(sprite);
	}

	pub fn remove_sprite(&mut self, sprite: ComponentHandle<Sprite<'a>>) {
		if let Some(index) = self.sprites.iter().position(|s| sprite == *s) {
			self.sprites.swap_remove(index);
		}
	}
}

pub struct RenderSystem;

impl<'a, S, E> System<S, E> for RenderSystem
where
	S: GameState<'a>,
	E: GameEvents<S, E>,
{
	fn update(s: &mut S, _e: &E) {
		let state = s.get_engine_state_mut();

		let sprites = &state.world.sprites;
		let render_state = &mut state.systems.render;

		render_state.sprites.sort_by(|ha, hb| {
			let a = sprites.get(*ha);
			let b = sprites.get(*hb);
			a.depth.cmp(&b.depth)
		});

		let mut canvas = state.sdl_context.canvas.borrow_mut();

		for h in &render_state.sprites {
			let sprite = sprites.get(*h);

			let image = &state.resources.images.get(sprite.image_resource);
			let query = image.texture.query();

			let transform = state.world.transforms.get(sprite.transform);
			let position = transform.position - image.center;

			canvas
				.copy(
					&image.texture,
					None,
					Rect::new(
						position.x as i32,
						position.y as i32,
						query.width,
						query.height,
					),
				)
				.unwrap();
		}
	}
}
