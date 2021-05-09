use crate::vec2::*;

use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::fmt;

pub type Rectf = Rect<f64>;
pub type Recti = Rect<i32>;

#[derive(Default, Clone, Copy)]
pub struct Rect<T> {
	/// Top-left
	pub pos: Vec2<T>,
	pub size: Vec2<T>,
}

impl<T: Copy> Rect<T> {
	pub fn new(pos: Vec2<T>, size: Vec2<T>) -> Rect<T> {
		Rect{pos, size}
	}

	pub fn new_xywh(x: T, y: T, width: T, height: T) -> Rect<T> {
		Rect{
			pos: Vec2::new(x, y),
			size: Vec2::new(width, height),
		}
	}
	
	pub fn left(&self) -> T {
		self.pos.x
	}
	
	pub fn top(&self) -> T {
		self.pos.y
	}
	
	pub fn top_left(&self) -> Vec2<T> {
		Vec2::new(self.left(), self.top())
	}
	
	pub fn width(&self) -> T {
		self.size.x
	}
	
	pub fn height(&self) -> T {
		self.size.y
	}
}

impl<T: Copy + Add<T, Output = T> + Sub<T, Output = T>> Rect<T> {
	pub fn right(&self) -> T {
		self.pos.x + self.size.x
	}
	
	pub fn bottom(&self) -> T {
		self.pos.y + self.size.y
	}
	
	pub fn top_right(&self) -> Vec2<T> {
		Vec2::new(self.right(), self.top())
	}
	
	pub fn bottom_left(&self) -> Vec2<T> {
		Vec2::new(self.left(), self.bottom())
	}
	
	pub fn bottom_right(&self) -> Vec2<T> {
		Vec2::new(self.right(), self.bottom())
	}
}
	
impl<T: Copy + Add<T, Output = T> + Sub<T, Output = T> + AddAssign + SubAssign> Rect<T> {
	pub fn set_left(&mut self, left: T) -> &mut Rect<T> {
		let diff = left - self.pos.x;
		self.pos.x = left;
		self.size.x -= diff;
		self
	}
	
	pub fn set_right(&mut self, right: T) -> &mut Rect<T> {
		let diff = right - self.pos.x - self.size.x;
		self.size.x += diff;
		self
	}
	
	pub fn set_top(&mut self, top: T) -> &mut Rect<T> {
		let diff = top - self.pos.y;
		self.pos.y = top;
		self.size.y -= diff;
		self
	}
	
	pub fn set_bottom(&mut self, bottom: T) -> &mut Rect<T> {
		let diff = bottom - self.pos.y - self.size.y;
		self.size.y += diff;
		self
	}
}

impl<T: PartialOrd + Copy + Add<T, Output = T> + Sub<T, Output = T>> Rect<T> {
	pub fn contains_vec_exclusive(&self, vec: Vec2<T>) -> bool {	
		vec.x > self.left() && vec.x < self.right() && vec.y > self.top() && vec.y < self.bottom()
	}
	
	pub fn contains_vec_inclusive(&self, vec: Vec2<T>) -> bool {
		vec.x >= self.left() && vec.x <= self.right() && vec.y >= self.top() && vec.y <= self.bottom()
	}
}

impl<T> fmt::Display for Rect<T> where T: fmt::Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<{}x{},{}x{}>", self.pos.x, self.pos.y, self.size.x, self.size.y)
	}
}

impl<T> fmt::Debug for Rect<T> where T: fmt::Debug {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "<{:?}x{:?},{:?}x{:?}>", self.pos.x, self.pos.y, self.size.x, self.size.y)
	}
}
