use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl Vector2 {
	pub fn one() -> Vector2 {
		Vector2 { x: 1.0, y: 1.0 }
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

impl<'a, 'b> Sub<&'b Vector2> for &'a Vector2 {
	type Output = Vector2;

	fn sub(self, other: &'b Vector2) -> Vector2 {
		Vector2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl<'a, 'b> Mul<&'b Vector2> for &'a Vector2 {
	type Output = Vector2;

	fn mul(self, other: &'b Vector2) -> Vector2 {
		Vector2 {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl<'a> Mul<f32> for &'a Vector2 {
	type Output = Vector2;

	fn mul(self, num: f32) -> Vector2 {
		Vector2 {
			x: self.x * num,
			y: self.y * num,
		}
	}
}

impl<'a, 'b> Div<&'b Vector2> for &'a Vector2 {
	type Output = Vector2;

	fn div(self, other: &'b Vector2) -> Vector2 {
		Vector2 {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl<'a> Div<f32> for &'a Vector2 {
	type Output = Vector2;

	fn div(self, num: f32) -> Vector2 {
		Vector2 {
			x: self.x * num,
			y: self.y * num,
		}
	}
}
