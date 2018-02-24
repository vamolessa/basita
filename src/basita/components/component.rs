use std::marker::PhantomData;
use std::fmt;
use std::slice::{Iter, IterMut};

pub trait Component: Default + fmt::Debug {}

#[derive(Default, Serialize, Deserialize)]
pub struct ComponentHandle<T: Component> {
	index: usize,

	#[serde(skip)]
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

impl<T: Component> PartialEq for ComponentHandle<T> {
	fn eq(&self, other: &Self) -> bool {
		self.index == other.index
	}
}

impl<T: Component> Eq for ComponentHandle<T> {}

impl<T: Component> fmt::Debug for ComponentHandle<T> {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "ComponentHandle [{}]", self.index)
	}
}

#[derive(Default, Serialize, Deserialize)]
pub struct ComponentMetadata<T: Component> {
	pub handle: ComponentHandle<T>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ComponentCollection<T: Component> {
	#[serde(skip)]
	indexes: Vec<usize>,

	components: Vec<T>,
	metadata: Vec<ComponentMetadata<T>>,
}

impl<T: Component> ComponentCollection<T> {
	pub fn init(&mut self) {}

	pub fn len(&self) -> usize {
		self.components.len()
	}

	pub fn add(&mut self, component: T) -> ComponentHandle<T> {
		let handle = ComponentHandle::new(self.indexes.len());

		self.indexes.push(self.components.len());
		self.components.push(component);
		self.metadata.push(ComponentMetadata { handle: handle });

		handle
	}

	pub fn get(&self, handle: ComponentHandle<T>) -> &T {
		let index = self.handle_to_index(handle);
		&self.components[index]
	}

	pub fn get_mut(&mut self, handle: ComponentHandle<T>) -> &mut T {
		let index = self.handle_to_index(handle);
		&mut self.components[index]
	}

	pub fn get_at(&self, index: usize) -> &T {
		&self.components[index]
	}

	pub fn get_handle(&self, index: usize) -> ComponentHandle<T> {
		self.metadata[index].handle
	}

	pub fn iter(&self) -> Iter<T> {
		self.components.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.components.iter_mut()
	}

	pub fn metadata_iter(&self) -> Iter<ComponentMetadata<T>> {
		self.metadata.iter()
	}

	fn handle_to_index(&self, handle: ComponentHandle<T>) -> usize {
		if handle.index < self.indexes.len() {
			self.indexes[handle.index]
		} else {
			panic!(
				"Could not find component\n{:?}\nfor handle {:?}",
				T::default(),
				handle
			)
		}
	}
}
