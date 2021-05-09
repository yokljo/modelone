use std::ops::Mul;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Colour {
	pub r: f64,
	pub g: f64,
	pub b: f64,
	pub a: f64,
}

impl Colour {
	pub fn rgb(r: f64, g: f64, b: f64) -> Colour {
		Colour{r, g, b, a: 1.}
	}
	
	pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Colour {
		Colour{r, g, b, a}
	}
	
	pub fn array(&self) -> [f64; 4] {
		[self.r, self.g, self.b, self.a]
	}
	
	pub fn array_f32(&self) -> [f32; 4] {
		[self.r as f32, self.g as f32, self.b as f32, self.a as f32]
	}
}

impl Mul<f64> for Colour {
	type Output = Colour;

	/// Multiply the RGB values of the colour by `rhs`, leaving the alpha value the same.
	fn mul(self, rhs: f64) -> Colour {
		Colour {
			r: self.r * rhs,
			g: self.g * rhs,
			b: self.b * rhs,
			a: self.a,
		}
	}
}

impl fmt::Debug for Colour {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RGBA<{},{},{},{}>", self.r, self.g, self.b, self.a)
	}
}

pub mod std_colour {
	use super::*;
	pub const BLACK: Colour = Colour{r: 0., g: 0., b: 0., a: 1.};
	pub const WHITE: Colour = Colour{r: 1., g: 1., b: 1., a: 1.};
	pub const RED: Colour = Colour{r: 1., g: 0., b: 0., a: 1.};
	pub const YELLOW: Colour = Colour{r: 1., g: 1., b: 0., a: 1.};
	pub const LIME: Colour = Colour{r: 0., g: 1., b: 0., a: 1.};
	pub const CYAN: Colour = Colour{r: 0., g: 1., b: 1., a: 1.};
	pub const BLUE: Colour = Colour{r: 0., g: 0., b: 1., a: 1.};
	pub const MAGENTA: Colour = Colour{r: 1., g: 0., b: 1., a: 1.};
}
