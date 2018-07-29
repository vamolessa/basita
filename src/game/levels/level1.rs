use basita::creator::Creator;
use basita::math::Vector2;

use entities::{block, player};

pub fn load<'a, 'b>(creator: &mut Creator<'a, 'b>) {
	player::new(creator, Vector2::new(100.0, 100.0));

	block::new(creator, Vector2::new(64.0, 64.0));
}
