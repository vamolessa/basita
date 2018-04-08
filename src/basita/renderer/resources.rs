use specs::Entity;

use core::assets::AssetCollection;
use super::assets::Image;

pub type ImageCollection = AssetCollection<Image>;

#[derive(Default)]
pub struct DirtySprites {
	pub entities: Vec<Entity>,
}