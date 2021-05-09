use std::ops::{Add, Sub};
use std::fmt;
use num;

pub type Vec2f = Vec2<f64>;
pub type Vec2f64 = Vec2<f64>;
pub type Vec2f32 = Vec2<f32>;
pub type Vec2i = Vec2<i32>;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	pub fn new(x: T, y: T) -> Vec2<T> {
		Vec2{x: x, y: y}
	}
}

impl<T: num::Num + PartialOrd> Vec2<T> {
	pub fn is_negative(self) -> bool {
		self.x < num::zero() && self.y < num::zero()
	}
	pub fn is_positive(self) -> bool {
		self.x > num::zero() && self.y > num::zero()
	}
}

impl<T> fmt::Display for Vec2<T> where T: fmt::Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<{}x{}>", self.x, self.y)
	}
}

impl<T> fmt::Debug for Vec2<T> where T: fmt::Debug {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<{:?}x{:?}>", self.x, self.y)
	}
}

/*impl<T> PartialEq for Vec2<T> where T: PartialEq {
	fn eq(&self, other: &Vec2<T>) -> bool {
		self.x == other.x && self.y == other.y
	}
}*/

impl<T> Add for Vec2<T> where T: Copy + Add<T, Output=T> {
	type Output = Vec2<T>;
	
	fn add(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2{x: self.x + rhs.x, y: self.y + rhs.y}
	}
}

impl<T> Sub for Vec2<T> where T: Copy + Sub<T, Output=T> {
	type Output = Vec2<T>;

	fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
		Vec2{x: self.x - rhs.x, y: self.y - rhs.y}
	}
}

/*impl<T> Mul<T> for Vec2<T> where
	T: Copy + Mul<T, Output=T>
{
	type Output = Vec2<T>;
	
	fn mul(self, scale: T) -> Vec2<T> {
		Vec2{x: self.x * scale, y: self.y * scale}
	}
}*/
