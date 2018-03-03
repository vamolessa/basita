use components::{Component, ComponentCollection};
use assets::{Asset, AssetCollection};

pub trait World {
	fn assets<T:Asset>(&self) -> &AssetCollection<T>;
	fn assets_mut<T:Asset>(&mut self) -> &mut AssetCollection<T>;

	fn components<T:Component>(&self) -> &ComponentCollection<T>;
	fn components_mut<T:Component>(&mut self) -> &mut ComponentCollection<T>;
}

pub trait WorldForComponent<T: Component> {
	fn get(&self) -> &ComponentCollection<T>;
	fn get_mut(&mut self) -> &mut ComponentCollection<T>;
}

pub trait WorldForAsset<T: Asset> {
	fn get(&self) -> &AssetCollection<T>;
	fn get_mut(&mut self) -> &mut AssetCollection<T>;
}