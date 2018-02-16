use super::super::EngineState;

pub trait System {
	fn update(&mut self, state: &mut EngineState) -> bool;
}
