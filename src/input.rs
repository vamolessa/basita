use sdl2::event::Event;

pub struct Input {}

impl Input {
	pub fn new() -> Input {
		Input {}
	}

	pub fn handle_event(&mut self, event: Event) -> bool {
		match event {
			_ => (),
		};

		true
	}
}
