pub trait Component {}

pub struct ComponentCollection<T>
where
	T: Component,
{
	pub all: Vec<T>,
}

impl<T> ComponentCollection<T>
where
	T: Component,
{
	pub fn new() -> Self {
		ComponentCollection { all: Vec::new() }
	}

	pub fn add(&mut self, component: T) {
		self.all.push(component);
	}
}
