use super::super::EngineState;

pub trait System {
	fn init(&mut self, _state: &mut EngineState) {}

	fn update(&mut self, state: &mut EngineState);
}
