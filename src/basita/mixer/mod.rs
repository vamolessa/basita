pub mod assets;
pub mod resources;
//pub mod components;
//pub mod systems;

mod mixer;
pub use self::mixer::*;

pub fn init(world: &mut specs::World) {
	world.add_resource(resources::Sfxs::default());
	world.add_resource(resources::Bgms::default());

	world.add_resource(resources::ChunkCommands::default());
	world.add_resource(resources::MusicCommands::default());
}
