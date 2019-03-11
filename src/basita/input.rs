use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use fxhash::FxHashMap;

#[derive(Default, Copy, Clone)]
pub struct State {
	pub is_pressed: bool,
	pub just_changed: bool,
}

impl State {
	pub fn just_pressed(&self) -> bool {
		self.is_pressed && self.just_changed
	}

	pub fn just_released(&self) -> bool {
		!self.is_pressed && self.just_changed
	}
}

#[derive(Default)]
pub struct Input {
	keys: FxHashMap<Keycode, State>,
}

impl Input {
	pub fn key(&self, keycode: Keycode) -> State {
		self.keys.get(&keycode).cloned().unwrap_or_default()
	}

	pub fn update(&mut self) {
		for (_keycode, state) in &mut self.keys {
			state.just_changed = false;
		}
	}

	pub fn handle_event(&mut self, event: Event) {
		match event {
			Event::KeyDown {
				keycode, repeat, ..
			} => {
				if !repeat {
					if let Some(keycode) = keycode {
						self.handle_key_change(keycode, true)
					}
				}
			}
			Event::KeyUp {
				keycode, repeat, ..
			} => {
				if !repeat {
					if let Some(keycode) = keycode {
						self.handle_key_change(keycode, false)
					}
				}
			}
			_ => (),
		};
	}

	fn handle_key_change(&mut self, keycode: Keycode, is_pressed: bool) {
		let key = self.keys.entry(keycode).or_default();
		key.is_pressed = is_pressed;
		key.just_changed = true;
	}
}
