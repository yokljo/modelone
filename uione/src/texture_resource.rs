#[macro_use] use crate::resources;
use crate::image_data::ImageData;
use crate::rect::Rect;

use std;
use std::any::Any;
use std::sync::Arc;
use std::fmt;

uione_graphic_resource!(TextureResource, get_texture_resource, TEXTURE_RESOURCE_HANDLE);

pub trait TextureHandle: Send + Sync + std::fmt::Debug {
	fn as_any(&self) -> &Any;
	
	fn blit<'image>(&self, image: &ImageData<'image>, rect: Rect<isize>) -> bool;
}

pub struct TextureResource {
	resource_impl: Box<TextureResourceImpl>,
}

impl fmt::Debug for TextureResource {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TextureResource")
	}
}

impl TextureResource {
	pub fn new(resource_impl: Box<TextureResourceImpl>) -> TextureResource {
		TextureResource {
			resource_impl,
		}
	}

	pub fn make_texture(&self, image_data: &ImageData) -> Arc<TextureHandle> {
		self.resource_impl.make_texture(image_data)
	}
}

pub trait TextureResourceImpl {
	fn make_texture(&self, image_data: &ImageData) -> Arc<TextureHandle>;
}
