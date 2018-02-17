use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
	buttons: Vec<ButtonState>,
}

impl Input {
	pub fn new() -> Self {
		Input {
			buttons: Vec::new(),
		}
	}

	pub fn new_button(&mut self, event_matcher: Keycode) -> Button {
		self.buttons.push(ButtonState {
			event_matcher: event_matcher,
			state: false,
			previous_state: false,
		});

		Button {
			index: self.buttons.len() - 1,
		}
	}

	/*
	pub fn set_event_matcher(&mut self, button: Button, event_matcher: Keycode) {
		self.buttons[button.index].event_matcher = event_matcher;
	}
	*/

	pub fn is_pressed(&self, button: Button) -> bool {
		self.buttons[button.index].state
	}

	pub fn was_pressed(&self, button: Button) -> bool {
		let button_state = &self.buttons[button.index];
		button_state.state && !button_state.previous_state
	}

	pub fn was_released(&self, button: Button) -> bool {
		let button_state = &self.buttons[button.index];
		!button_state.state && button_state.previous_state
	}

	pub fn update(&mut self) {
		for button in &mut self.buttons {
			button.previous_state = button.state;
		}
	}

	pub fn handle_event(&mut self, event: Event) {
		match event {
			Event::KeyDown { keycode, .. } => if let Some(keycode) = keycode {
				self.handle_button_change(keycode, true)
			},
			Event::KeyUp { keycode, .. } => if let Some(keycode) = keycode {
				self.handle_button_change(keycode, false)
			},
			_ => (),
		};
	}

	fn handle_button_change(&mut self, keycode: Keycode, state: bool) {
		for button in &mut self.buttons {
			if button.event_matcher == keycode {
				button.state = state;
			}
		}
	}
}

struct ButtonState {
	pub event_matcher: Keycode,
	pub state: bool,
	pub previous_state: bool,
}

#[derive(Clone, Copy)]
pub struct Button {
	index: usize,
}
