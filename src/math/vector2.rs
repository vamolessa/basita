use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl Vector2 {
	pub fn zero() -> Vector2 {
		Vector2 { x: 0.0, y: 0.0 }
	}

	pub fn sqr_magnitude(&self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	pub fn magnitude(&self) -> f32 {
		self.sqr_magnitude().sqrt()
	}
}

impl<'a, 'b> Add<&'b Vector2> for &'a Vector2 {
	type Output = Vector2;

	fn add(self, other: &'b Vector2) -> Vector2 {
		Vector2 {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}
