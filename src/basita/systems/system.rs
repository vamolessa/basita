use World;

pub trait System {
	fn init(&mut self, &mut World) {}
	fn update(&mut self, &mut World);
}
