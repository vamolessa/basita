use super::assets::{Bgm, Sfx};
use core::assets::AssetCollection;

pub type Sfxs = AssetCollection<Sfx>;
pub type Bgms = AssetCollection<Bgm>;

pub enum ChunkVariant {
	Play,
	Pan(u8, u8),
}

pub struct ChunkCommand {
	pub chunk_index: usize,
	pub variant: ChunkVariant,
}

#[derive(Default)]
pub struct ChunkCommands {
	pub commands: Vec<ChunkCommand>,
}

impl ChunkCommands {
	pub fn add_play(&mut self, chunk_index: usize) {
		self.commands.push(ChunkCommand {
			chunk_index: chunk_index,
			variant: ChunkVariant::Play,
		});
	}

	pub fn add_pan(&mut self, chunk_index: usize, left: u8, right: u8)
	{
		self.commands.push(ChunkCommand {
			chunk_index: chunk_index,
			variant: ChunkVariant::Pan(left, right),
		})
	}
}

#[derive(Default)]
pub struct MusicCommands {
	pub commands: Vec<()>,
}
