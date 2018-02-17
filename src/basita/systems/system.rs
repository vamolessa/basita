use super::super::EngineState;

pub trait System {
	fn init(&self, _state: &mut EngineState) {}

	fn update(&self, state: &mut EngineState);
}
