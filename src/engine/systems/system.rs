use super::super::EngineState;

pub trait System {
	fn update(&mut self, engine: &mut EngineState);
}
