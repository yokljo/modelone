use crate::vec2::Vec2;
use crate::cast_slice::cast_boxed_slice;

use std::fmt;
use std::borrow::Cow;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImageFormat {
	R8,
	RGB888,
	RGBA8888,
}

impl ImageFormat {
	pub fn bytes_per_pixel(&self) -> usize {
		match *self {
			ImageFormat::R8 => 1,
			ImageFormat::RGB888 => 3,
			ImageFormat::RGBA8888 => 4,
		}
	}
}

#[derive(Clone, PartialEq)]
pub struct ImageData<'data> {
	data: Cow<'data, [u8]>,
	size: Vec2<usize>,
	format: ImageFormat,
}

impl<'data> fmt::Debug for ImageData<'data> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "ImageData({:?}, {:?})", self.size, self.format)
	}
}

impl<'data> ImageData<'data> {
	/// Create a new ImageData. Returns None if the data has the wrong size for the given `size` and
	/// `format`.
	pub fn new(data: Vec<u8>, size: Vec2<usize>, format: ImageFormat) -> Option<ImageData<'static>> {
		let expected_data_size = size.x * size.y * format.bytes_per_pixel();
		if data.len() != expected_data_size {
			return None;
		}
	
		Some(ImageData {
			data: Cow::Owned(data), size, format
		})
	}
	
	pub fn new_borrowed(data: &'data [u8], size: Vec2<usize>, format: ImageFormat) -> Option<ImageData<'data>> {
		let expected_data_size = size.x * size.y * format.bytes_per_pixel();
		if data.len() != expected_data_size {
			return None;
		}
	
		Some(ImageData {
			data: Cow::Borrowed(data), size, format
		})
	}
	
	pub fn new_zero(size: Vec2<usize>, format: ImageFormat) -> ImageData<'static> {
		let expected_data_size = size.x * size.y * format.bytes_per_pixel();
		ImageData::new(vec![0; expected_data_size], size, format).unwrap()
	}
	
	pub fn new_rgb888(data: Vec<(u8, u8, u8)>, size: Vec2<usize>) -> Option<ImageData<'static>> {
		let data_raw = unsafe { cast_boxed_slice(data.into_boxed_slice()) }.into_vec();
		ImageData::new(data_raw, size, ImageFormat::RGB888)
	}
	
	pub fn new_rgba8888(data: Vec<(u8, u8, u8, u8)>, size: Vec2<usize>) -> Option<ImageData<'static>> {
		let data_raw = unsafe { cast_boxed_slice(data.into_boxed_slice()) }.into_vec();
		ImageData::new(data_raw, size, ImageFormat::RGBA8888)
	}
	
	pub fn data(&self) -> &[u8] {
		&self.data
	}
	
	pub fn size(&self) -> Vec2<usize> {
		self.size
	}
	
	pub fn format(&self) -> ImageFormat {
		self.format
	}
}
