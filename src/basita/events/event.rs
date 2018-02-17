use std::rc::Rc;

use systems::{SystemCollection, SystemHandle};

pub struct Event {
	system_handles: Vec<SystemHandle>,
}

impl Event {
	pub fn new() -> Self {
		Event {
			system_handles: Vec::new(),
		}
	}

	pub fn raise(&self, systems: Rc<SystemCollection>) {
		for handle in &self.system_handles {
			let _system = systems.get(handle.clone());
		}
	}
}
