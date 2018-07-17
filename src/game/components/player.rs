use basita::specs::{Component, VecStorage};

#[derive(Default, Clone, Copy, Debug)]
pub struct Player;

impl Component for Player {
	type Storage = VecStorage<Self>;
}
