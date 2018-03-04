use std::any::TypeId;
use std::marker::PhantomData;
use std::fmt;

use super::{SparseStorage};

pub trait Component: Default {}

#[derive(Default, Serialize, Deserialize)]
pub struct ComponentHandle<T: Component> {
	entity_id: usize,

	#[serde(skip)]
	_phantom: PhantomData<T>,
}

impl<T: Component> ComponentHandle<T> {
	fn new(entity_id: usize) -> Self {
		ComponentHandle {
			entity_id: entity_id,
			_phantom: PhantomData,
		}
	}
}

impl<T: Component> Clone for ComponentHandle<T> {
	fn clone(&self) -> Self {
		ComponentHandle::new(self.entity_id)
	}
}

impl<T: Component> Copy for ComponentHandle<T> {}

impl<T: Component> PartialEq for ComponentHandle<T> {
	fn eq(&self, other: &Self) -> bool {
		self.entity_id == other.entity_id
	}
}

impl<T: Component> Eq for ComponentHandle<T> {}

impl<T: 'static + Component> fmt::Debug for ComponentHandle<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"ComponentHandle [{}] for type {:?}",
			self.entity_id,
			TypeId::of::<T>(),
		)
	}
}

#[derive(Default, Serialize, Deserialize)]
pub struct ComponentCollection<T: Component> {
	components: SparseStorage<T>,
}

impl<T: 'static + Component> ComponentCollection<T> {
	pub fn init(&mut self) {}

	pub fn add(&mut self, component: T) -> ComponentHandle<T> {
		let entity_id = self.components.add(component);
		ComponentHandle::new(entity_id)
	}

	/*
	pub fn get(&self, handle: ComponentHandle<T>) -> &T {
		self.components.get(handle.entity_id)
	}

	pub fn get_mut(&mut self, handle: ComponentHandle<T>) -> &mut T {
		self.components.get_mut(handle.entity_id)
	}
	*/

	/*
	pub fn iter(&self) -> SparseStorageIter<T> {
		self.components.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.components.iter_mut()
	}

	fn handle_to_index(&self, handle: ComponentHandle<T>) -> usize {
		if handle.index < self.indexes.len() {
			self.indexes[handle.index]
		} else {
			panic!(
				"Could not find component of type {:?} for handle {:?}",
				TypeId::of::<T>(),
				handle
			)
		}
	}
	*/
}
