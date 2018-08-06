use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl Vector2 {
	pub fn new(x: f32, y: f32) -> Self {
		Vector2 { x: x, y: y }
	}

	pub fn zero() -> Self {
		Vector2 { x: 0.0, y: 0.0 }
	}

	pub fn dot(a: Self, b: Self) -> f32 {
		a.x * b.x + a.y * b.y
	}

	pub fn scale(a: Self, b: Self) -> Self {
		Vector2 {
			x: a.x * b.x,
			y: a.y * b.y,
		}
	}

	pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}

	pub fn sqr_magnitude(self) -> f32 {
		self.x * self.x + self.y * self.y
	}

	pub fn magnitude(self) -> f32 {
		self.sqr_magnitude().sqrt()
	}

	pub fn normalized(self) -> Self {
		let magnitude = self.magnitude();
		Vector2 {
			x: self.x / magnitude,
			y: self.y / magnitude,
		}
	}
}

impl Add<Vector2> for Vector2 {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Vector2 {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl AddAssign<Vector2> for Vector2 {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
	}
}

impl Sub<Vector2> for Vector2 {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Vector2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl SubAssign<Vector2> for Vector2 {
	fn sub_assign(&mut self, other: Self) {
		self.x -= other.x;
		self.y -= other.y;
	}
}

impl Neg for Vector2 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Vector2 {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl Mul<f32> for Vector2 {
	type Output = Self;

	fn mul(self, value: f32) -> Self::Output {
		Vector2 {
			x: self.x * value,
			y: self.y * value,
		}
	}
}

impl MulAssign<f32> for Vector2 {
	fn mul_assign(&mut self, value: f32) {
		self.x *= value;
		self.y *= value;
	}
}

impl Div<f32> for Vector2 {
	type Output = Self;

	fn div(self, value: f32) -> Self::Output {
		Vector2 {
			x: self.x / value,
			y: self.y / value,
		}
	}
}

impl DivAssign<f32> for Vector2 {
	fn div_assign(&mut self, value: f32) {
		self.x /= value;
		self.y /= value;
	}
}
