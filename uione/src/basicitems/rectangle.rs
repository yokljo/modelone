use crate::item::*;
use crate::colour::*;
use crate::geometry::{Geometry, BasicGeometry, GeometryDamageFlags};
use crate::resources::GraphicResourceManager;
#[macro_use] use crate::shader::*;

use modelone::{impl_changeable_struct, dispatch_struct_update};
use modelone::object::{ApplyContext, Object};
use modelone::change_value::ValueChange;

uione_uniform_struct! {
	struct Vert {
		pos: [f32; 2] = "vPos",
		colour: [f32; 4] = "vColour",
		border: i32 = "vBorder",
	}
}

uione_uniform_struct! {
	struct Locals {
		gradient: f32 = "uGradient",
	}
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
	pub item_data: ItemData,
	pub colour: Colour,
	pub border_width: f64,
	pub border_colour: Colour,
}

impl Rectangle {
	pub fn new_colour(colour: Colour) -> Rectangle {
		Rectangle {
			item_data: ItemData::new(),
			colour,
			border_width: 0.,
			border_colour: std_colour::BLACK,
		}
	}
}

impl Item for Rectangle {
	impl_get_item!(item_data);
	
	fn update_geometry(&self, _old_geometry: Option<Box<Geometry>>, _resource_manager: &mut GraphicResourceManager) -> Option<Box<Geometry>> {
		// Rectangle scenegraph implementation goes here
		//let rect_geom = old_geometry.as_type::<BasicGeometry::<BasicShader<Vert, Locals>>>().unwrap();
		
		let shader = Box::new(BasicShader::<Vert, Locals>::new(
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/rect_150.glsl")).into(),
		));
		
		if self.border_width < 1. {
			let col = self.colour.array_f32();
			
			let item = self.get_item();
			Some(Box::new(BasicGeometry::<BasicShader<Vert, Locals>> {
				vertices: vec![
					Vert{pos: [0., 0.], colour: col, border: 0},
					Vert{pos: [(item.size.x) as f32, 0. as f32], colour: col, border: 0},
					Vert{pos: [(item.size.x) as f32, (item.size.y) as f32], colour: col, border: 0},
					Vert{pos: [0., (item.size.y) as f32], colour: col, border: 0},
				],
				indices: vec![
					0, 1, 2, 2, 3, 0,
				],
				size: item.size,
				uniform: Locals {
					gradient: 100.
				},
				shader,
				damage_flags: GeometryDamageFlags::EVERYTHING,
			}))
		} else {
			let col = self.colour.array_f32();
			let bcol = self.border_colour.array_f32();
			
			let item = self.get_item();
			
			let real_x_border_width = self.border_width.min(item.size.x / 2.) as f32;
			let real_y_border_width = self.border_width.min(item.size.y / 2.) as f32;
			
			Some(Box::new(BasicGeometry::<BasicShader<Vert, Locals>> {
				vertices: vec![
					Vert{pos: [real_x_border_width, real_y_border_width], colour: col, border: 0},
					Vert{pos: [(item.size.x as f32 - real_x_border_width) as f32, real_y_border_width], colour: col, border: 0},
					Vert{pos: [(item.size.x as f32 - real_x_border_width) as f32, (item.size.y as f32 - real_y_border_width) as f32], colour: col, border: 0},
					Vert{pos: [real_x_border_width, (item.size.y as f32 - real_y_border_width) as f32], colour: col, border: 0},
					
					Vert{pos: [0., 0.], colour: bcol, border: 1},
					Vert{pos: [item.size.x as f32, 0.], colour: bcol, border: 1},
					Vert{pos: [item.size.x as f32, item.size.y as f32], colour: bcol, border: 1},
					Vert{pos: [0., item.size.y as f32], colour: bcol, border: 1},
					
					Vert{pos: [real_x_border_width, real_y_border_width], colour: bcol, border: 1},
					Vert{pos: [(item.size.x as f32 - real_x_border_width) as f32, real_y_border_width], colour: bcol, border: 1},
					Vert{pos: [(item.size.x as f32 - real_x_border_width) as f32, (item.size.y as f32 - real_y_border_width) as f32], colour: bcol, border: 1},
					Vert{pos: [real_x_border_width, (item.size.y as f32 - real_y_border_width) as f32], colour: bcol, border: 1},
				],
				indices: vec![
					0, 1, 2, 2, 3, 0,
					
					4, 5, 9, 9, 8, 4,
					5, 6, 10, 10, 9, 5,
					6, 7, 11, 11, 10, 6,
					7, 4, 8, 8, 11, 7,
				],
				size: item.size,
				uniform: Locals {
					gradient: 1.
				},
				shader,
				damage_flags: GeometryDamageFlags::EVERYTHING,
			}))
		}
	}
}

impl_changeable_struct!{RectangleChange[RectangleSignal] for Rectangle:
	item_data: ItemDataChange,
	colour: ValueChange<Colour>,
	border_width: ValueChange<f64>,
	border_colour: ValueChange<Colour>,
}

impl Object<RectangleChange> for Rectangle {
	fn update(&self, cxt: &mut ApplyContext<RectangleChange>, signal: &RectangleSignal) {
		dispatch_struct_update!{RectangleChange[RectangleSignal] for self, cxt, signal:
			item_data: ItemData,
		}
	}
}
