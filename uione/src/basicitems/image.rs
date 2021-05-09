use crate::item::*;
use crate::image_data::ImageData;
use crate::geometry::{Geometry, BasicGeometry, GeometryDamageFlags};
use crate::resources::GraphicResourceManager;
use crate::texture_resource::{get_texture_resource, TextureHandle};
use crate::shader::*;
#[macro_use] use crate::shader;

use modelone::{impl_changeable_struct, dispatch_struct_update};
use modelone::object::{ApplyContext, Object};

use std::sync::Arc;

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
pub struct Image {
	pub item_data: ItemData,
	pub source: Arc<ImageData<'static>>,
}

impl Item for Image {
	impl_get_item!(item_data);
	
	fn update_geometry(&self, _old_geometry: Option<Box<Geometry>>, resource_manager: &mut GraphicResourceManager) -> Option<Box<Geometry>> {
		let shader = Box::new(BasicShader::<Vert, Locals>::new(
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/image.glsl")).into(),
		));
		let texture_resource = get_texture_resource(resource_manager).unwrap();
		let texture = texture_resource.make_texture(&self.source);
		
		let item = self.get_item();
		Some(Box::new(BasicGeometry::<BasicShader<Vert, Locals>> {
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
				tex: texture,
			},
			shader,
			damage_flags: GeometryDamageFlags::EVERYTHING,
		}))
	}
}

impl_changeable_struct!{ImageChange[ImageSignal] for Image:
	item_data: ItemDataChange,
}

impl Object<ImageChange> for Image {
	fn update(&self, cxt: &mut ApplyContext<ImageChange>, signal: &ImageSignal) {
		dispatch_struct_update!{ImageChange[ImageSignal] for self, cxt, signal:
			item_data: ItemData,
		}
	}
}
