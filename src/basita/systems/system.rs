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

	pub fn add_system(
		&mut self,
		init: Option<fn(&mut S, &mut E) -> ()>,
		update: fn(&mut S, &E) -> (),
	) {
		if let Some(init_function) = init {
			self.inits.push(init_function);
		}

		self.updates.push(update);
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
