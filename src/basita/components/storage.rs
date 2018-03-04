#[derive(Serialize, Deserialize)]
pub struct SparseStorage<T> {
	elements: Vec<Option<T>>,
}

impl<T> SparseStorage<T> {
	pub fn set(&mut self, index: usize, element: T) {
		if index >= self.elements.len() {
			self.elements.resize_default(index + 1);
		}

		self.elements[index] = Some(element);
	}

	pub fn get(&self, index: usize) -> Option<&T> {
		self.elements[index].as_ref()
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.elements[index].as_mut()
	}

	pub fn clear(&mut self) {
		self.elements.clear();
	}
}

impl<T> Default for SparseStorage<T> {
	fn default() -> Self {
		SparseStorage {
			elements: Vec::default(),
		}
	}
}
