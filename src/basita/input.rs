use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use fxhash::FxHashMap;

#[derive(Default)]
pub struct State {
	pub is_pressed: bool,
	pub just_changed: bool,
}

#[derive(Default)]
pub struct Input {
	pub keys: FxHashMap<Keycode, State>,
}

impl Input {
	pub fn update(&mut self) {
		for (_keycode, state) in &mut self.keys {
			if state.just_changed {
				state.is_pressed = !state.is_pressed;
				state.just_changed = false;
			}
		}
	}

	pub fn handle_event(&mut self, event: Event) {
		match event {
			Event::KeyDown { keycode, .. } => if let Some(keycode) = keycode {
				self.handle_key_change(keycode, true)
			},
			Event::KeyUp { keycode, .. } => if let Some(keycode) = keycode {
				self.handle_key_change(keycode, false)
			},
			_ => (),
		};
	}

	fn handle_key_change(&mut self, keycode: Keycode, is_pressed: bool) {
		let key = self.keys.entry(keycode).or_insert(State::default());
		key.is_pressed = is_pressed;
		key.just_changed = true;
	}
}
