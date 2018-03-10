#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct EntityHandle {
	id: usize,
}

impl EntityHandle {
	pub fn id(&self) -> usize {
		self.id
	}
}

#[derive(Default, Serialize, Deserialize)]
pub struct Entities {
	len: usize,
}

impl Entities {
	pub fn len(&self) -> usize {
		self.len
	}

	pub fn add(&mut self) -> EntityHandle {
		let id = self.len;
		self.len += 1;

		EntityHandle { id: id }
	}

	pub fn clear(&mut self) {
		self.len = 0;
	}
}
