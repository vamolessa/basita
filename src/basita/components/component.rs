use std::marker::PhantomData;
use std::fmt;

pub trait Component {}

#[derive(Serialize, Deserialize)]
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

impl<T: Component> Clone for ComponentHandle<T> {
	fn clone(&self) -> Self {
		ComponentHandle::new(self.index)
	}
}

impl<T: Component> Copy for ComponentHandle<T> {}

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
		ComponentCollection { all: Vec::new() }
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

	pub fn get_handle(&self, index: usize) -> ComponentHandle<T> {
		ComponentHandle::new(index)
	}

	pub fn handle_iter(&self) -> ComponentCollectionHandleIterator<T> {
		ComponentCollectionHandleIterator::new(ComponentHandle::new(self.all.len()))
	}
}

pub struct ComponentCollectionHandleIterator<T: Component> {
	previous_handle: ComponentHandle<T>,
}

impl<T: Component> ComponentCollectionHandleIterator<T> {
	pub fn new(handle: ComponentHandle<T>) -> Self {
		ComponentCollectionHandleIterator {
			previous_handle: handle,
		}
	}
}

impl<T: Component> Iterator for ComponentCollectionHandleIterator<T> {
	type Item = ComponentHandle<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.previous_handle.index > 0 {
			self.previous_handle.index -= 1;
			Some(self.previous_handle)
		} else {
			None
		}
	}
}
