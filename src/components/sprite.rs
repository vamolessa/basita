use image_resources::Image;

pub struct Sprite<'a> {
	pub x: i32,
	pub y: i32,
	pub image: &'a Image<'a>,
}

pub struct Sprites<'a> {
	pub all: Vec<Sprite<'a>>,
}

impl<'a> Sprites<'a> {
	pub fn new() -> Sprites<'a> {
		Sprites { all: Vec::new() }
	}

	pub fn add(&mut self, sprite: Sprite<'a>) -> &Sprite<'a> {
		self.all.push(sprite);
		&self.all[self.all.len() - 1]
	}
}
