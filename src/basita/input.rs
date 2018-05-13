use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(Default)]
pub struct State {
	pub is_pressed: bool,
	pub just_changed: bool,
}

#[derive(Default)]
pub struct Input {
	pub keys: Vec<State>,
}

impl Input {
	pub fn new() -> Self {
		let key_count = (Keycode::Sleep as usize) + 1;
		//let mut keys = Vec::with_capacity(key_count);
		//keys.resize_default(key_count);

		println!("key_count: {}", key_count );

		Input {
			keys: Default::default(),
		}
	}

	pub fn update(&mut self) {
		for key in &mut self.keys {
			if key.just_changed {
				key.is_pressed = !key.is_pressed;
				key.just_changed = false;
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
		let key = &mut self.keys[keycode as usize];
		key.is_pressed = is_pressed;
		key.just_changed = true;
	}
}
