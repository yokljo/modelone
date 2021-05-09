use crate::image_data::{ImageData, ImageFormat};
use crate::resources::{self, GraphicResourceManager, GraphicResource, GraphicResourceError};
use crate::texture_resource::{TextureResource, TextureHandle};
use crate::vec2::Vec2;
use crate::rect::Rect;

use std::fmt;
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rusttype;
use rusttype::gpu_cache::{Cache, CacheBuilder};
use font_loader::system_fonts;
use lazy_static::lazy_static;

uione_graphic_resource!(FontResource, get_font_resource, FONT_RESOURCE_HANDLE);

#[derive(Clone)]
pub struct FontEntry {
	id: usize,
	pub font: Arc<rusttype::Font<'static>>,
}

//#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
//pub struct FontHandle(usize);

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FontDetails {
	pub family: String,
	pub italic: bool,
	pub oblique: bool,
	pub bold: bool,
	pub monospace: bool,
}

impl FontDetails {
	pub fn new(family: String) -> FontDetails {
		FontDetails {
			family,
			italic: false,
			oblique: false,
			bold: false,
			monospace: false,
		}
	}
}

pub struct FontStore {
	loaded_fonts: Vec<Arc<rusttype::Font<'static>>>,
	font_details_mapping: HashMap<FontDetails, usize>,
}

impl FontStore {
	fn new() -> FontStore {
		FontStore {
			loaded_fonts: vec![],
			font_details_mapping: HashMap::new(),
		}
	}
	
	fn add_font(&mut self, details: FontDetails, font: Arc<rusttype::Font<'static>>) -> FontEntry {
		let id = self.loaded_fonts.len();
		self.font_details_mapping.insert(details, id);
		self.loaded_fonts.push(font.clone());
		
		FontEntry {
			id,
			font,
		}
	}
	
	pub fn font_for_details(&mut self, details: FontDetails) -> FontEntry {
		if let Some(entry_index) = self.font_details_mapping.get(&details) {
			return FontEntry {
				id: *entry_index,
				font: self.loaded_fonts[*entry_index].clone(),
			};
		}
		
		let mut info_builder = system_fonts::FontPropertyBuilder::new()
			.family(&details.family);
		
		if details.italic { info_builder = info_builder.italic(); }
		if details.oblique { info_builder = info_builder.oblique(); }
		if details.bold { info_builder = info_builder.bold(); }
		if details.monospace { info_builder = info_builder.monospace(); }
		
		let font_info = info_builder.build();
		
		let font_data = system_fonts::get(&font_info).unwrap().0;
		//for font_name in system_fonts::query_specific(font_info) {}
		let font = rusttype::Font::from_bytes(font_data).unwrap();
		
		self.add_font(details, Arc::new(font))
	}
}

lazy_static! {
	pub static ref STATIC_FONT_STORE: Mutex<FontStore> = Mutex::new(FontStore::new());
}

/*struct GlyphIdWithXPos {
	id: usize,
	sub_pixel_offset: i8,
}

struct GlyphCache {
	// Mapping from glyphs that need to be drawn on the texture atlas, to the rectangle of where the
	// glyph should be drawn on the atlas.
	pending_glyphs: Hash<GlyphIdWithXPos, Rect<u32>>,
	atlas_width: u32,
	atlas_height: u32,
	current_row_height: u32,
	current_x: u32,
	current_y: u32,
}

impl GlyphCache {
	fn process_pending(&mut self) {
		for (glyphidpos, rect) in self.pending_glyphs.drain() {
			
		}
	}
}*/

pub struct FontResource {
	pub cache: Cache<'static>,
	pub tex: Option<Arc<TextureHandle>>,
}

impl fmt::Debug for FontResource {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "FontResource")
	}
}

impl FontResource {
	pub fn new() -> FontResource {
		let cache = CacheBuilder {
			width: 256,
			height: 256,
			scale_tolerance: 0.1,
			position_tolerance: 0.1,
			pad_glyphs: true,
		}.build();
		
		FontResource {
			cache,
			tex: None,
		}
	}
	
	pub fn ensure_tex(&mut self, texture_resource: &mut TextureResource) -> Arc<TextureHandle> {
		if self.tex.is_none() {
			let (w, h) = self.cache.dimensions();
			let image = ImageData::new_zero(Vec2::new(w as usize, h as usize), ImageFormat::RGBA8888);
			self.tex = Some(texture_resource.make_texture(&image));
		}
		
		let tex = self.tex.as_mut().unwrap();
		
		tex.clone()
	}
	
	pub fn queue_glyph(&mut self, font: &FontEntry, positioned_glyph: rusttype::PositionedGlyph<'static>) {
		self.cache.queue_glyph(font.id, positioned_glyph);
	}
	
	pub fn cache_queued(&mut self, texture_resource: &mut TextureResource) {
		let tex = self.ensure_tex(texture_resource);
	
		self.cache.cache_queued(|rect, pixel_data| {
			let image_data = ImageData::new_borrowed(pixel_data, Vec2::new(rect.width() as usize, rect.height() as usize), ImageFormat::R8).unwrap();
			tex.blit(&image_data, Rect::new_xywh(rect.min.x as isize, rect.min.y as isize, rect.width() as isize, rect.height() as isize));
		}).unwrap();
	}
	
	pub fn get_glyph_rect(&self, font: &FontEntry, positioned_glyph: &rusttype::PositionedGlyph<'static>) -> Option<Rect<f32>> {
		match self.cache.rect_for(font.id, positioned_glyph) {
			Ok(Some((glyph_rect, _dest_rect))) => {
				Some(Rect::new_xywh(glyph_rect.min.x, glyph_rect.min.y, glyph_rect.width(), glyph_rect.height()))
			}
			_ => None
		}
	}
}
