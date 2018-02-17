use super::super::EngineState;

pub trait System {
	fn init(&self, _state: &mut EngineState) {}

	fn update(&self, state: &mut EngineState);
}

#[derive(Clone, Copy)]
pub struct SystemHandle {
	index: usize,
}

pub struct SystemCollection<'a> {
	pub all: Vec<Box<'a + System>>,
}

impl<'a> SystemCollection<'a> {
	pub fn new() -> Self {
		SystemCollection { all: Vec::new() }
	}

	pub fn add<T: System + 'a>(&mut self, system: T) -> SystemHandle {
		self.all.push(Box::new(system));

		SystemHandle {
			index: self.all.len() - 1,
		}
	}

	pub fn get(&self, handle: SystemHandle) -> &Box<'a + System> {
		&self.all[handle.index]
	}
}
