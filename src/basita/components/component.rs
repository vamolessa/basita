use std::marker::PhantomData;
use std::fmt;
use std::slice::{Iter, IterMut};

pub trait Component: Default {}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct ComponentHandle<T: Component> {
	index: usize,
	_phantom: PhantomData<T>,
}

impl<T: Component> ComponentHandle<T> {
	fn new(index: usize) -> Self {
		ComponentHandle {
			index: index,
			_phantom: PhantomData,
		}
	}
}

impl<T: Component> fmt::Debug for ComponentHandle<T> {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "ComponentHandle [{}]", self.index)
	}
}

pub struct ComponentCollection<T: Component> {
	pub all: Vec<T>,
}

impl<T: Component> ComponentCollection<T> {
	pub fn new() -> Self {
		let mut collection = Vec::new();
		collection.push(T::default());

		ComponentCollection { all: collection }
	}

	pub fn len(&self) -> usize {
		self.all.len()
	}

	pub fn add(&mut self, component: T) -> ComponentHandle<T> {
		let handle = ComponentHandle::new(self.all.len());
		self.all.push(component);
		handle
	}

	pub fn get(&self, handle: ComponentHandle<T>) -> &T {
		&self.all[handle.index]
	}

	pub fn get_mut(&mut self, handle: ComponentHandle<T>) -> &mut T {
		&mut self.all[handle.index]
	}

	pub fn get_at(&self, index: usize) -> &T {
		&self.all[index]
	}

	pub fn get_handle(&self, index: usize) -> ComponentHandle<T> {
		ComponentHandle::new(index)
	}

	pub fn iter(&self) -> Iter<T> {
		self.all.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.all.iter_mut()
	}
}
