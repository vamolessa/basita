pub struct Event<S, E, D>
where
	D: Copy,
{
	callbacks: Vec<fn(&mut S, &E, D) -> ()>,
}

impl<S, E, D> Event<S, E, D>
where
	D: Copy,
{
	pub fn new() -> Self {
		Event {
			callbacks: Vec::new(),
		}
	}

	pub fn subscribe(&mut self, callback: fn(&mut S, &E, D) -> ()) {
		self.callbacks.push(callback);
	}

	pub fn raise(&self, state: &mut S, events: &E, data: D) {
		for callback in &self.callbacks {
			callback(state, events, data);
		}
	}
}

impl<S, E, D> Default for Event<S, E, D>
where
	D: Copy,
{
	fn default() -> Self {
		Event::new()
	}
}
