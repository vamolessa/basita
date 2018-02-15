use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
	buttons: Vec<Button>,
}

impl Input {
	pub fn new() -> Input {
		Input {
			buttons: Vec::new(),
		}
	}

	pub fn handle_event(&mut self, event: Event) -> bool {
		match event {
			Event::KeyDown {
				keycode: keycode, ..
			} => (),
			Event::KeyUp {
				keycode: keycode, ..
			} => (),
			_ => (),
		};

		true
	}

	fn handle_button_change(keycode: Keycode, state: bool) {
		
	}
}

pub struct Button {
	pub event_matcher: Keycode,
	pub state: bool,
	pub previous_state: bool,
}

impl Button {
	pub fn was_pressed(&self) -> bool {
		self.state && !self.previous_state
	}

	pub fn was_released(&self) -> bool {
		!self.state && self.previous_state
	}
}
