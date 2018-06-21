use basita::specs::System;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
	type SystemData = ();

	fn run(&mut self, (): Self::SystemData) {}
}
