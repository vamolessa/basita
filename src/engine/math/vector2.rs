use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl Vector2 {
	pub fn new(x: f32, y: f32) -> Vector2 {
		Vector2 { x: x, y: y }
	}

	/*
	pub fn one() -> Vector2 {
		Vector2 { x: 1.0, y: 1.0 }
	}

	pub fn sqr_magnitude(&self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	pub fn magnitude(&self) -> f32 {
		self.sqr_magnitude().sqrt()
	}
	*/
}

impl From<(f32, f32)> for Vector2 {
	fn from((x, y): (f32, f32)) -> Vector2 {
		Vector2 { x: x, y: y }
	}
}

impl Add<Vector2> for Vector2 {
	type Output = Vector2;

	fn add(self, other: Vector2) -> Vector2 {
		Vector2 {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl Sub<Vector2> for Vector2 {
	type Output = Vector2;

	fn sub(self, other: Vector2) -> Vector2 {
		Vector2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl Mul<Vector2> for Vector2 {
	type Output = Vector2;

	fn mul(self, other: Vector2) -> Vector2 {
		Vector2 {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl Mul<f32> for Vector2 {
	type Output = Vector2;

	fn mul(self, num: f32) -> Vector2 {
		Vector2 {
			x: self.x * num,
			y: self.y * num,
		}
	}
}

impl Div<Vector2> for Vector2 {
	type Output = Vector2;

	fn div(self, other: Vector2) -> Vector2 {
		Vector2 {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
}

impl Div<f32> for Vector2 {
	type Output = Vector2;

	fn div(self, num: f32) -> Vector2 {
		Vector2 {
			x: self.x * num,
			y: self.y * num,
		}
	}
}
