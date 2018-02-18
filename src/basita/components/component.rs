use std::fmt;
use std::marker::PhantomData;

pub trait Component {}

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

impl<T: Component> PartialEq for ComponentHandle<T> {
	fn eq(&self, other: &Self) -> bool {
		true
	}
}

impl<T: Component> Eq for ComponentHandle<T> {}

impl<T: Component> Clone for ComponentHandle<T> {
	fn clone(&self) -> Self {
		ComponentHandle::new(self.index)
	}
}

impl<T: Component> Copy for ComponentHandle<T> {}

impl<T: Component> Default for ComponentHandle<T> {
	fn default() -> Self {
		ComponentHandle {
			index: Default::default(),
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

	pub fn iter(&self) -> ComponentCollectionIter<T> {
		ComponentCollectionIter {
			collection: self,
			current: 0,
		}
	}
}

pub struct ComponentCollectionIter<'a, T>
where
	T: 'a + Component,
{
	collection: &'a ComponentCollection<T>,
	current: usize,
}

impl<'a, T> Iterator for ComponentCollectionIter<'a, T>
where
	T: 'a + Component,
{
	type Item = ComponentHandle<T>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current < self.collection.all.len() {
			let result = Some(ComponentHandle::new(self.current));
			self.current += 1;
			result
		} else {
			None
		}
	}
}
