use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::fmt;
use std::slice::{Iter, IterMut};

use serde::{Serialize, Serializer};

use uuid::Uuid;

pub trait Component: Default + Serialize {}

#[derive(Default, Deserialize)]
pub struct ComponentHandle<T: Component> {
	id: Uuid,
	_phantom: PhantomData<T>,
}

impl<T: Component> ComponentHandle<T> {
	pub fn new(id: Uuid) -> Self {
		ComponentHandle {
			id: id,
			_phantom: PhantomData,
		}
	}
}

impl<T: Component> Clone for ComponentHandle<T> {
	fn clone(&self) -> Self {
		ComponentHandle::new(self.id)
	}
}

impl<T: Component> Copy for ComponentHandle<T> {}

impl<T: Component> PartialEq for ComponentHandle<T> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl<T: Component> Eq for ComponentHandle<T> {}

impl<T: Component> Hash for ComponentHandle<T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl<T: Component> fmt::Debug for ComponentHandle<T> {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "ComponentHandle [{}]", self.id)
	}
}

#[derive(Default, Deserialize)]
pub struct ComponentCollection<T: Component> {
	index_map: HashMap<ComponentHandle<T>, usize>,
	components: Vec<(ComponentHandle<T>, T)>,
}

impl<T: Component> ComponentCollection<T> {
	pub fn len(&self) -> usize {
		self.components.len()
	}

	pub fn add(&mut self, component: T) -> ComponentHandle<T> {
		let id = Uuid::new_v4();
		let handle = ComponentHandle::new(id);

		self.index_map.insert(handle, self.components.len());
		self.components.push((handle, component));

		handle
	}

	pub fn get(&self, handle: &ComponentHandle<T>) -> &T {
		let index = self.index_map.get(handle).cloned().unwrap();
		&self.components[index].1
	}

	pub fn get_mut(&mut self, handle: &ComponentHandle<T>) -> &mut T {
		let index = self.index_map.get(handle).cloned().unwrap();
		&mut self.components[index].1
	}

	pub fn get_at(&self, index: usize) -> &T {
		&self.components[index].1
	}

	pub fn get_handle(&self, index: usize) -> ComponentHandle<T> {
		self.components[index].0
	}

	pub fn iter(&self) -> Iter<(ComponentHandle<T>, T)> {
		self.components.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<(ComponentHandle<T>, T)> {
		self.components.iter_mut()
	}
}

impl<T: Component> Serialize for ComponentCollection<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.components.serialize(serializer)
	}
}

impl<T: Component> Serialize for ComponentHandle<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.id.serialize(serializer)
	}
}
