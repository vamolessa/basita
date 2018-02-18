use super::super::EngineState;

pub struct Event<D> {
	callbacks: Vec<fn(&mut EngineState, &D) -> ()>,
}

impl<D> Event<D> {
	pub fn subscribe(&mut self, callback: fn(&mut EngineState, &D) -> ()) {
		self.callbacks.push(callback);
	}

	pub fn raise(&self, state: &mut EngineState, data: &D) {
		for callback in &self.callbacks {
			callback(state, data);
		}
	}
}

impl<D> Default for Event<D> {
	fn default() -> Self {
		Event {
			callbacks: Vec::new(),
		}
	}
}
