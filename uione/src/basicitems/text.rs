#![allow(dead_code)]
use crate::item::*;
use crate::colour::*;
use crate::image_data::ImageData;
use crate::geometry::{Geometry, BasicGeometry, BasicFnGeometry, GeometryDamageFlags};
use crate::resources::GraphicResourceManager;
use crate::texture_resource::{get_texture_resource, TextureHandle};
use crate::font_resource;
#[macro_use] use crate::shader::*;
use crate::vec2::*;

use modelone::{impl_changeable_struct, dispatch_struct_update};
use modelone::object::{ApplyContext, Object};
use modelone::change_value::ValueChange;

use std;
use std::sync::Arc;
use rusttype;
use font_loader::system_fonts;

// https://github.com/PistonDevelopers/conrod/blob/master/src/backend/gfx.rs

/*enum Capitalisation {
	Mixed,
	Upper,
	Lower,
	SmallCaps,
}*/

#[derive(Debug, PartialEq, Clone)]
pub struct FontDesc {
	point_size: f64,
	family: Arc<String>,
	bold: bool,
	italic: bool,
	underline: bool,
	strike_out: bool,
	//capitalisation: Capitalisation,
}

impl FontDesc {
	pub fn new(point_size: f64, family: Arc<String>) -> FontDesc {
		FontDesc {
			point_size,
			family,
			bold: false,
			italic: false,
			underline: false,
			strike_out: false,
			//capitalisation: Capitalisation::Mixed,
		}
	}
}

#[derive(Clone)]
pub struct TextFormat {
	pub font: rusttype::Font<'static>,//FontDesc,
	pub colour: Colour,
}

impl TextFormat {
	pub fn new_size_family_colour(_point_size: f64, family: Arc<String>, colour: Colour) -> TextFormat {
		let font_info = system_fonts::FontPropertyBuilder::new()
			.family(&*family)
			.build();
		let font_data = system_fonts::get(&font_info).unwrap().0;
		//for font_name in system_fonts::query_specific(font_info) {}
		let font = rusttype::Font::from_bytes(font_data).unwrap();
		
		TextFormat {
			//font: FontDesc::new(point_size, family),
			font,
			colour,
		}
	}
}

impl PartialEq for TextFormat {
	fn eq(&self, _rhs: &TextFormat) -> bool {
		false
	}
}

impl std::fmt::Debug for TextFormat {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "TextFormat")
	}
}

/*struct Run {
	format: TextFormat,
	text: String,
}

struct Line {
	runs: Vec<Run>,
}

enum WrapMode {
	WordBoundary,
	Anywhere,
	WordBoundaryOrAnywhere
}

struct TextLayout {
	lines: Vec<Line>,
	wrap_mode: WrapMode,
}


struct GlyphGeom {
	glyph: GlyphId,
	pos: Vec2<f32>,
	size: Vec2<f32>,
	rotation: f32,
	source_index: usize,
}

trait Layout {
	fn layout_text(&self, text_layout: &TextLayout) -> Vec<GlyphGeom> {
		vec![]
	}
}*/

pub struct LineRunLayout {
	glyphs: Vec<rusttype::GlyphId>,
	height: f32,
	spacing: f32,
	character_index: usize,
}

pub enum RunLayout {
	Line(LineRunLayout),
}

struct LineLayout {
	runs: Vec<RunLayout>,
}

pub struct TextLayout {
	lines: Vec<LineLayout>,
}

impl TextLayout {
	pub fn new() -> TextLayout {
		TextLayout {
			lines: vec![],
		}
	}
}

struct BasicLayouter {
	char_height: f32,
	line_spacing: f32,
	character_spacing: f32,
	format: TextFormat,
}

impl BasicLayouter {
	fn layout(&self, _text: &str) -> TextLayout {
		let mut glyphs = vec![];
		let mut commit = || {
			let mut line_glyphs = vec![];
			std::mem::swap(&mut line_glyphs, &mut glyphs);
			LineRunLayout {
				glyphs: line_glyphs,
				height: self.char_height,
				spacing: self.character_spacing,
				character_index: 0,
			}
		};
		
		/*for (i, c) in text.chars().enumerate() {
			if c == '\n' {
				commit();
			} else {
				let glyph_id = rusttype::GlyphId(if c >= self.format.font.glyph_count() {
					0
				} else {
					c
				});
				
				glyphs.push(glyph_id);
			}
		}*/
		
		commit();
		
		TextLayout::new()
	}
}

uione_uniform_struct! {
	struct Vert {
		pos: [f32; 2] = "vPos",
		tex_coord: [f32; 2] = "vTexCoord",
	}
}

uione_uniform_struct! {
	struct Locals {
		tex: Arc<TextureHandle> = "uTex",
	}
}

#[derive(Debug, PartialEq)]
pub struct Text {
	pub item_data: ItemData,
	pub format: Arc<TextFormat>,
	pub text: Arc<String>,
}

impl Text {
	pub fn new(format: Arc<TextFormat>) -> Text {
		Text {
			item_data: ItemData::new(),
			format,
			text: Arc::new(String::new()),
		}
	}
}

struct TextGeomState {
	text: Arc<String>,
	font_entry: font_resource::FontEntry,
}

impl TextGeomState {
	fn on_draw(&mut self, geometry: &mut BasicGeometry<BasicShader<Vert, Locals>>, resource_manager: &mut GraphicResourceManager) {
		//let texture_resource = get_texture_resource(resource_manager).unwrap();
		let mut font_res = font_resource::get_font_resource(resource_manager).unwrap();
		let mut tex_res = get_texture_resource(resource_manager).unwrap();
		// List of (resource id, the actual reference to that resource).
		//let res: [(usize, Option<&mut Any>)] = [(0, None), (1, None)];
		// This actually gets the references to the resources with the requested IDs.
		//resource_manager.get_resources(res);
		//let mut font_res = res[0].downcast_mut<FontResource>();
		//let mut tex_res = res[1].downcast_mut<TexResource>();
		
		// Simplified as:
		/*uione_get_resources!(
			let font_res = font_resource::Thing;
			let tex_res = tex_resource::Thing;
		);*/
		
		/*for glyph in self.font_entry.font.layout(&*self.text, rusttype::Scale::uniform(geometry.size.y as f32), rusttype::Point{x: 0., y: 0.}) {
			/*let bb = if let Some(bb) = glyph.pixel_bounding_box() {
				bb
			} else {
				continue;
			};*/
			//font_res.queue_glyph(self.font_handle, glyph);
		}*/
		
		let size = geometry.size();
		for c in self.text.chars() {
			let glyph = self.font_entry.font.glyph(c)
				.scaled(rusttype::Scale::uniform(size.y as f32))
				.positioned(rusttype::Point{x: 0., y: 0.});
				
			font_res.queue_glyph(&self.font_entry, glyph);
		}
		
		font_res.cache_queued(&mut tex_res);
		
		geometry.vertices = vec![];
		geometry.indices = vec![];
		
		let mut x_pos = 0.;
		for c in self.text.chars() {
			let glyph = self.font_entry.font.glyph(c)
				.scaled(rusttype::Scale::uniform(size.y as f32))
				.positioned(rusttype::Point{x: 0., y: 0.});
			let glyph_width = size.y as f32;
			
			if let Some(glyph_rect) = font_res.get_glyph_rect(&self.font_entry, &glyph) {
				let index_off = geometry.vertices.len() as u32;
				geometry.vertices.push(Vert{pos: [x_pos, 0.], tex_coord: [glyph_rect.left(), glyph_rect.top()]});
				geometry.vertices.push(Vert{pos: [x_pos + glyph_width as f32, 0. as f32], tex_coord: [glyph_rect.right(), glyph_rect.top()]});
				geometry.vertices.push(Vert{pos: [x_pos + glyph_width as f32, size.y as f32], tex_coord: [glyph_rect.right(), glyph_rect.bottom()]});
				geometry.vertices.push(Vert{pos: [x_pos, size.y as f32], tex_coord: [glyph_rect.left(), glyph_rect.bottom()]});
				geometry.indices.push(index_off);
				geometry.indices.push(index_off + 1);
				geometry.indices.push(index_off + 2);
				geometry.indices.push(index_off + 2);
				geometry.indices.push(index_off + 3);
				geometry.indices.push(index_off);
			}
			
			x_pos += glyph_width;
		}
		
		/*geometry.vertices = vec![
			Vert{pos: [0., 0.], tex_coord: [0., 0.]},
			Vert{pos: [size.x as f32, 0. as f32], tex_coord: [1., 0.]},
			Vert{pos: [size.x as f32, size.y as f32], tex_coord: [1., 1.]},
			Vert{pos: [0., size.y as f32], tex_coord: [0., 1.]},
		];
		geometry.indices = vec![
			
		];*/
		geometry.damage_flags |= GeometryDamageFlags::VERTEX_DATA;
	}
}

impl Item for Text {
	impl_get_item!(item_data);
	
	// TODO: This function should be somehow set to be called by the scenegraph with "damage"
	// information at the moment before it is about to be drawn, that way it can ensure all the
	// required glyphs are in the glyph texture cache.
	fn update_geometry(&self, old_geometry: Option<Box<Geometry>>, resource_manager: &mut GraphicResourceManager) -> Option<Box<Geometry>> {
		if old_geometry.is_some() {
			return old_geometry;
		}
		
		// Rectangle scenegraph implementation goes here
		//let rect_geom = old_geometry.as_type::<BasicGeometry::<BasicShader<Vert, Locals>>>().unwrap();
		let shader = Box::new(BasicShader::<Vert, Locals>::new(
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/text.glsl")).into(),
		));
		
		//font_resource::get_font_resource(resource_manager).unwrap().add_thing("Cool beans".into());
		//println!("{:?}", font_resource::get_font_resource(resource_manager).unwrap().stuff);
		
		let item = self.get_item();
		
		if item.size.is_positive() {
			let w = item.size.x as usize;
			let h = item.size.y as usize;
			
			let mut source_data = vec![(255u8, 255, 255, 255); w * h];
			
			for glyph in self.format.font.layout(&*self.text, rusttype::Scale::uniform(item.size.y as f32), rusttype::Point{x: 0., y: 0.}) {
				let bb = if let Some(bb) = glyph.pixel_bounding_box() {
					bb
				} else {
					continue;
				};
				glyph.draw(|x, y, v| {
					let mut sx = x as i32;
					let mut sy = y as i32;
					sx += bb.min.x;
					sy += bb.min.y;
					sy += h as i32;
					if sx >= 0 && sy >= 0 {
						let index = sx as usize + sy as usize*w;
						if index < source_data.len() {
							let brightness = 255 - ((v * 255.) as u8);
							source_data[index] = (brightness, brightness, brightness, 255);
						}
					}
				});
			}
			
			let mut texture_resource = get_texture_resource(resource_manager).unwrap();
			let mut font_res = font_resource::get_font_resource(resource_manager).unwrap();
			/*let source = ImageData::new_rgba8888(source_data, Vec2::new(w, h)).unwrap();
			
			let texture = texture_resource.make_texture(&source);*/
			let texture = font_res.ensure_tex(&mut texture_resource);
			
			let font_entry = font_resource::STATIC_FONT_STORE.lock().unwrap().font_for_details(font_resource::FontDetails::new("Arial".into()));
			
			Some(Box::new(BasicFnGeometry {
				basic_geometry: BasicGeometry::<BasicShader<Vert, Locals>> {
					vertices: vec![
						Vert{pos: [0., 0.], tex_coord: [0., 0.]},
						Vert{pos: [(item.size.x) as f32, 0. as f32], tex_coord: [1., 0.]},
						Vert{pos: [(item.size.x) as f32, (item.size.y) as f32], tex_coord: [1., 1.]},
						Vert{pos: [0., (item.size.y) as f32], tex_coord: [0., 1.]},
					],
					indices: vec![
						0, 1, 2, 2, 3, 0,
					],
					size: item.size,
					uniform: Locals {
						tex: texture.clone(),
					},
					shader,
					damage_flags: GeometryDamageFlags::EVERYTHING,
				},
				on_draw_arg: TextGeomState {
					text: self.text.clone(),
					font_entry,
				},
				on_draw_fn: TextGeomState::on_draw,
			}))
		} else {
			None
		}
	}
}

impl_changeable_struct!{TextChange[TextSignal] for Text:
	item_data: ItemDataChange,
	format: ValueChange<Arc<TextFormat>>,
	text: ValueChange<Arc<String>> => on_changed (&mut _text) {
		//println!(text
	},
}

impl Object<TextChange> for Text {
	fn update(&self, cxt: &mut ApplyContext<TextChange>, signal: &TextSignal) {
		dispatch_struct_update!{TextChange[TextSignal] for self, cxt, signal:
			item_data: ItemData,
		}
	}
}
