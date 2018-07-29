use sdl::{SdlContext, SdlStorage};
use specs::World;

pub struct Creator<'a: 'b, 'b> {
	pub world: &'b mut World,
	pub sdl_context: &'a SdlContext,
	pub sdl_storage: &'b SdlStorage<'a>,
}
