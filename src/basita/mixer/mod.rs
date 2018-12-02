pub mod assets;
pub mod resources;

mod mixer;
pub use self::mixer::*;

pub fn init(world: &mut specs::World) {
	world.add_resource(resources::Sfxs::default());
	world.add_resource(resources::Bgms::default());

	world.add_resource(resources::SfxCommands::default());
	world.add_resource(resources::MusicCommands::default());
}
