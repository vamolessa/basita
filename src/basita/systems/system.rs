use super::super::EngineState;

pub trait System {
	fn init(&mut self, _state: &mut EngineState) {}

	fn update(&mut self, state: &mut EngineState);

	fn test<'b>(&self, _systems: &'b mut super::super::EngineSystems, _:&'b ()) {}
}
