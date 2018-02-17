use super::super::{EngineEvents, EngineState};

pub trait System {
	fn init(_state: &mut EngineState, _events: &mut EngineEvents) {}
	fn update(state: &mut EngineState, events: &EngineEvents);
}

pub struct SystemCollection {
	inits: Vec<fn(&mut EngineState, &mut EngineEvents) -> ()>,
	updates: Vec<fn(&mut EngineState, &EngineEvents) -> ()>,
}

impl SystemCollection {
	pub fn new() -> Self {
		SystemCollection {
			inits: Vec::new(),
			updates: Vec::new(),
		}
	}

	pub fn add<S>(&mut self)
	where
		S: System,
	{
		self.inits.push(S::init);
		self.updates.push(S::update);
	}

	pub fn init(&self, state: &mut EngineState, events: &mut EngineEvents) {
		for f in &self.inits {
			f(state, events);
		}
	}

	pub fn update(&self, state: &mut EngineState, events: &EngineEvents) {
		for f in &self.updates {
			f(state, events);
		}
	}
}
