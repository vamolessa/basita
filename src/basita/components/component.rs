pub trait Component {}

declare_handle!(ComponentHandle);

pub struct ComponentCollection<T: Component> {
	pub all: Vec<T>,
}

impl<T: Component> ComponentCollection<T> {
	pub fn new() -> Self {
		ComponentCollection { all: Vec::new() }
	}

	pub fn add(&mut self, component: T) -> ComponentHandle<T> {
		self.all.push(component);
		ComponentHandle::new(self.all.len() - 1)
	}

	pub fn get(&self, handle: ComponentHandle<T>) -> &T {
		&self.all[handle.index]
	}

	pub fn get_mut(&mut self, handle: ComponentHandle<T>) -> &mut T {
		&mut self.all[handle.index]
	}

	pub fn get_handle(&self, index: usize) -> ComponentHandle<T> {
		ComponentHandle::new(index)
	}
}
