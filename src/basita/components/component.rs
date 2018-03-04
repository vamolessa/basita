use super::SparseStorage;
use EntityHandle;

pub trait Component: Default {}

#[derive(Default, Serialize, Deserialize)]
pub struct ComponentCollection<T: Component> {
	components: SparseStorage<T>,
}

impl<T: Component> ComponentCollection<T> {
	pub fn init(&mut self) {}

	pub fn set(&mut self, entity: EntityHandle, component: T) {
		self.components.set(entity.id(), component);
	}

	pub fn get(&self, entity: EntityHandle) -> Option<&T> {
		self.components.get(entity.id())
	}

	pub fn get_mut(&mut self, entity: EntityHandle) -> Option<&mut T> {
		self.components.get_mut(entity.id())
	}

	pub fn at(&self, entity_index: usize) -> Option<&T> {
		self.components.get(entity_index)
	}

	pub fn clear(&mut self) {
		self.components.clear();
	}
}
