use specs::Entity;

use core::assets::AssetCollection;
use super::assets::Image;

pub type ImageCollection = AssetCollection<Image>;

#[derive(Default)]
pub struct UpdatedSprites {
	pub entities: Vec<Entity>,
}