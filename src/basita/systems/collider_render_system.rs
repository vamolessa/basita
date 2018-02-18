use sdl2::rect::Rect;
use sdl2::pixels::Color;

use super::super::{ContainsEngineEvents, ContainsEngineState};

pub fn update<'a, S, E>(s: &mut S, _e: &E)
where
	S: ContainsEngineState<'a>,
	E: ContainsEngineEvents,
{
	let state = s.get_engine_state_mut();

	let mut canvas = state.sdl_context.canvas.borrow_mut();

	for box_collider in &state.box_colliders.all {
		let transform = state.transforms.get(box_collider.transform);

		canvas.set_draw_color(Color::RGBA(0, 255, 0, 100));
		canvas
			.draw_rect(Rect::new(
				transform.position.x as i32,
				transform.position.y as i32,
				box_collider.size.x as u32,
				box_collider.size.x as u32,
			))
			.unwrap();
	}
}
