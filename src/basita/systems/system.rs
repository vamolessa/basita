pub trait System<S, E> {
	fn init(&mut S, &mut E) {}
	fn update(&mut S, &E);
}

pub struct SystemCollection<S, E> {
	inits: Vec<fn(&mut S, &mut E) -> ()>,
	updates: Vec<fn(&mut S, &E) -> ()>,
}

impl<S, E> SystemCollection<S, E> {
	pub fn new() -> Self {
		SystemCollection {
			inits: Vec::new(),
			updates: Vec::new(),
		}
	}

	pub fn add_system<T>(&mut self)
	where
		T: System<S, E>,
	{
		self.inits.push(T::init);
		self.updates.push(T::update);
	}

	pub fn init(&self, state: &mut S, events: &mut E) {
		for f in &self.inits {
			f(state, events);
		}
	}

	pub fn update(&self, state: &mut S, events: &E) {
		for f in &self.updates {
			f(state, events);
		}
	}
}
