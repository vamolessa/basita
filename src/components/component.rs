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
	pub fn new() -> ComponentCollection<T> {
		ComponentCollection { all: Vec::new() }
	}

	pub fn add(&mut self, component: T) -> &T {
		self.all.push(component);
		&self.all[self.all.len() - 1]
	}
}
