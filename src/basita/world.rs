use components::{Component, ComponentCollection};

pub trait World {
	fn components<T>(&self) -> &ComponentCollection<T>
	where
		T: Component;

	fn components_mut<T>(&mut self) -> &mut ComponentCollection<T>
	where
		T: Component;
}

pub trait WorldForComponent<T: Component> {
	fn get(&self) -> &ComponentCollection<T>;
	fn get_mut(&mut self) -> &mut ComponentCollection<T>;
}
