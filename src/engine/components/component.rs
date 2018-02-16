use std::marker::PhantomData;

pub trait Component {}

pub struct ComponentHandle<T: Component> {
	index: usize,
	_phantom: PhantomData<T>,
}

impl<T: Component> Clone for ComponentHandle<T> {
	fn clone(&self) -> Self {
		ComponentHandle {
			index: self.index,
			_phantom: self._phantom,
		}
	}
}

impl<T: Component> Copy for ComponentHandle<T> {}

pub struct ComponentCollection<T: Component> {
	pub all: Vec<T>,
}

impl<T: Component> ComponentCollection<T> {
	pub fn new() -> Self {
		ComponentCollection { all: Vec::new() }
	}

	pub fn add(&mut self, component: T) -> ComponentHandle<T> {
		self.all.push(component);

		ComponentHandle {
			index: self.all.len() - 1,
			_phantom: PhantomData,
		}
	}

	pub fn get(&self, handle: ComponentHandle<T>) -> &T {
		&self.all[handle.index]
	}

	pub fn get_mut(&mut self, handle: ComponentHandle<T>) -> &mut T {
		&mut self.all[handle.index]
	}
}
