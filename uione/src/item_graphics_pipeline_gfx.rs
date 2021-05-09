use crate::image_data;
use crate::cast_slice::cast_slice;
use crate::item;
use crate::item::*;
use crate::geometry::{self, GeometryDamageFlags};
use crate::resources::{self, GraphicResourceManager, GraphicResource, GraphicResourceError, GraphicResourceHandle};
use crate::texture_resource::{TextureResource, TextureResourceImpl, TextureHandle};
use crate::shader::Shader;
use crate::shadervalue::ShaderValueType;
use crate::vec2::Vec2f;
use crate::rect::{Rect, Rectf};

use std;
use std::rc::Rc;
use std::sync::{Arc, mpsc};
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::os::raw::c_void;
use std::any::Any;
use std::cell::{Cell, RefCell, RefMut};

use cgmath;
use gl;
use gl::types::{GLuint, GLint, GLenum, GLsizei, GLboolean};

use modelone::IdAlloc;
use modelone::object::{ApplyHandle, JustSignalChange};
use modelone::change_value::ValueChange;
use modelone::change_option::OptionChange;

uione_uniform_struct! {
	/// Uniforms that apply to all Item shaders.
	struct ItemLocals {
		transform: [[f32; 4]; 4] = "uTransform",
		pos: [f32; 2] = "uPos",
		size: [f32; 2] = "uSize",
	}
}

struct ItemCacheEntry {
	parent: Option<usize>,
	//children: Vec<usize>,
	apply_handle: Option<ApplyHandle<item::ItemDataChange>>,
	pos: Vec2f,
	global_pos: Vec2f,
	size: Vec2f,
	geometry: Option<Box<geometry::Geometry>>,
	vertex_array: Option<GlVertexArray>,
	vertex_buffer: Option<GlArrayBuffer>,
	//uniform_buffer: Option<GlArrayBuffer>,
	index_buffer: Option<GlIndexBuffer>,
	shader_program: Option<Rc<ResolvedShaderProgram>>,
	is_mouse_area: bool,
}

impl ItemCacheEntry {
	/*fn new() -> ItemCacheEntry {
		ItemCacheEntry {
			parent: None,
			//children: vec![],
			apply_handle: None,
			pos: Vec2f::new(0., 0.),
			global_pos: Vec2f::new(0., 0.),
			size: Vec2f::new(0., 0.),
			geometry: None,
			vertex_array: None,
			vertex_buffer: None,
			//uniform_buffer: None,
			index_buffer: None,
			shader_program: None,
			is_mouse_area: false,
		}
	}*/
	
	fn process_geometry(&mut self, shader_cache: &mut ShaderCache) {
		if let Some(ref mut geometry) = self.geometry {
			let damage_flags = geometry.damage_flags();
			
			if damage_flags != GeometryDamageFlags::NONE {
				let shader = geometry.shader();
				
				let shader_program = shader_cache.get_shader_program(shader).unwrap();
				
				let vertex_array = GlVertexArray::new();
				vertex_array.bind();
				
				let vertex_buffer = GlArrayBuffer::new(geometry.vertex_data(), gl::ARRAY_BUFFER, gl::STATIC_DRAW);
				
				let index_buffer = GlIndexBuffer::new(geometry.indices());
				
				self.vertex_array = Some(vertex_array);
				self.shader_program = Some(shader_program);
				self.vertex_buffer = Some(vertex_buffer);
				self.index_buffer = Some(index_buffer);
			}
			
			geometry.reset_damage_flags();
		}
	}

	fn draw(&mut self, transform: &cgmath::Matrix4<f32>, resource_manager: &mut GraphicResourceManager, shader_cache: &mut ShaderCache) {
		if let Some(ref mut geometry) = self.geometry {
			geometry.on_draw(resource_manager);
		}
		
		self.process_geometry(shader_cache);
		
		if let (Some(ref mut geometry), Some(vertex_array), Some(vertex_buffer), Some(index_buffer), Some(shader_program))
				= (&mut self.geometry, &self.vertex_array, &self.vertex_buffer, &self.index_buffer, &self.shader_program) {
			//geometry.on_draw(resource_manager);
			//self.process_geometry(shader_cache);
			
			shader_program.program.use_program();
			vertex_array.bind();
			vertex_buffer.bind();
			
			let item_locals = ItemLocals {
				transform: (*transform).into(),
				pos: [self.global_pos.x as f32, self.global_pos.y as f32],
				size: [(self.size.x / geometry.size().x) as f32, (self.size.y / geometry.size().y) as f32],
			};
			
			let item_locals_slice = &[item_locals];
			let raw_item_locals = unsafe { cast_slice(item_locals_slice) };
			let item_uniform_buffer = GlArrayBuffer::new(raw_item_locals, gl::UNIFORM_BUFFER, gl::DYNAMIC_DRAW);
			
			//let uniform_buffer = GlArrayBuffer::new(geometry.uniform_data(), gl::UNIFORM_BUFFER, gl::DYNAMIC_DRAW);
			
			unsafe {
				//println!(": {}, {}", shader_program.uniform_locals, shader_program.uniform_item_locals);
				gl::BindBufferBase(gl::UNIFORM_BUFFER, shader_program.uniform_item_locals, item_uniform_buffer.handle);
				//gl::BindBufferBase(gl::UNIFORM_BUFFER, shader_program.uniform_locals, uniform_buffer.handle);
				let mut field_id = 0;
				for field in &shader_program.uniform_locals_fields {
					field.process_uniform(geometry.uniform_data(), field_id);
					field_id += 1;
				}
			}
			
			shader_program.use_as_vertex_buffer();
			
			index_buffer.draw();
			/*let item_locals = ItemLocals {
				transform: (*transform).into(),
				pos: [self.global_pos.x as f32, self.global_pos.y as f32],
				size: [(self.size.x / geometry.size().x) as f32, (self.size.y / geometry.size().y) as f32],
			};
			
			let uniforms = GeometryUniforms {
				uniform_data: geometry.uniform_data(),
				shader: geometry.shader(),
			};
			
			let vertex_format = &geometry.shader().vertex_format();
			println!("{:?}", vertex_format);*/
			
			//let vertex_source = glium::vertex::VerticesSource::VertexBuffer(vertex_buffer.as_slice_any(), vertex_format, false);
			
			//surface.draw(vertex_source, index_buffer, shader_program, &uniforms, &Default::default()).unwrap();
			
			/*glVertexAttribPointer(
				0, // attribute 0. No particular reason for 0, but must match the layout in the shader.
				3, // size
				GL_FLOAT,           // type
				GL_FALSE,           // normalized?
				0,                  // stride
				(void*)0            // array buffer offset
			);*/
			
			//encoder.update_constant_buffer(&pipe_data.item_locals, &item_locals);
			
			//encoder.draw(&slice, &pso, pipe_data);
		}
	}
}

struct ItemCache {
	alloc: IdAlloc<RefCell<ItemCacheEntry>>,
	main_item: Option<usize>,
}

impl ItemCache {
	fn new() -> ItemCache {
		ItemCache {
			alloc: IdAlloc::new(),
			main_item: None,
		}
	}
	
	/*fn set_main_item(&mut self, id: Option<usize>) {
		self.main_item = id;
	}*/
	
	fn draw_worker(&self, transform: &cgmath::Matrix4<f32>, id: usize, resource_manager: &mut GraphicResourceManager, shader_cache: &mut ShaderCache) {
		{
			let entry = self.alloc.get(id);
			entry.borrow_mut().draw(transform, resource_manager, shader_cache);
		}
		
		self.alloc.apply_to_all(&mut |entry_id, entry: &RefCell<ItemCacheEntry>| {
			let parent = entry.borrow_mut().parent;
			if let Some(parent_id) = parent {
				if parent_id == id {
					self.draw_worker(transform, entry_id, resource_manager, shader_cache);
				}
			}
		});
	}
	
	fn draw(&self, transform: &cgmath::Matrix4<f32>, resource_manager: &mut GraphicResourceManager, shader_cache: &mut ShaderCache) {
		if let Some(id) = self.main_item {
			self.draw_worker(transform, id, resource_manager, shader_cache);
		}
	}
	
	fn hover_item_helper(&self, pos: Vec2f, parent_id: Option<usize>) -> Option<usize> {
		let mut result_id = None;
		
		self.alloc.apply_to_all(&mut |id, item_cache_entry_cell: &RefCell<ItemCacheEntry>| {
			let item_cache_entry = item_cache_entry_cell.borrow();
			if item_cache_entry.parent == parent_id {
				if let Some(found_id) = self.hover_item_helper(pos, Some(id)) {
					result_id = Some(found_id);
				} else if item_cache_entry.is_mouse_area {
					let global_rect = Rectf::new(item_cache_entry.global_pos, item_cache_entry.size);
					if global_rect.contains_vec_exclusive(pos) {
						result_id = Some(id);
					}
				}
			}
		});
		
		result_id
	}
	
	pub fn hover_item(&self, pos: Vec2f) -> Option<usize> {
		self.hover_item_helper(pos, None)
	}
}

fn vertex_size_and_type(shader_value_type: &ShaderValueType) -> Option<(usize, GLenum)> {
	use self::ShaderValueType::*;
	match shader_value_type {
		Int => Some((1, gl::INT)),
		UInt => Some((1, gl::UNSIGNED_INT)),
		Float => Some((1, gl::FLOAT)),
		//Double => Some((1, gl::DOUBLE)),
		Vec2 => Some((2, gl::FLOAT)),
		Vec3 => Some((3, gl::FLOAT)),
		Vec4 => Some((4, gl::FLOAT)),
		IntVec2 => Some((2, gl::INT)),
		IntVec3 => Some((3, gl::INT)),
		IntVec4 => Some((4, gl::INT)),
		Mat2 => Some((2, gl::FLOAT)),
		Mat3 => Some((3, gl::FLOAT)),
		Mat4 => Some((4, gl::FLOAT)),
		_ => None,
	}
}

struct VertexFieldInfo {
	attrib_location: GLuint,
	size: GLint,
	shader_value_type: ShaderValueType,
	field_type: GLenum,
	offset: *const c_void,
}

impl VertexFieldInfo {
	fn process_uniform(&self, data: &[u8], id: u32) {
		use crate::shadervalue::ShaderValueType::*;
		unsafe {
			let field_ptr = data.as_ptr().offset(self.offset as isize);
			match self.shader_value_type {
				Int => gl::Uniform1iv(self.attrib_location as i32, 1, field_ptr as *const _),
				UInt => gl::Uniform1uiv(self.attrib_location as i32, 1, field_ptr as *const _),
				Float => gl::Uniform1fv(self.attrib_location as i32, 1, field_ptr as *const _),
				Vec2 => gl::Uniform2fv(self.attrib_location as i32, 1, field_ptr as *const _),
				Vec3 => gl::Uniform3fv(self.attrib_location as i32, 1, field_ptr as *const _),
				Vec4 => gl::Uniform4fv(self.attrib_location as i32, 1, field_ptr as *const _),
				IntVec2 => gl::Uniform2iv(self.attrib_location as i32, 1, field_ptr as *const _),
				IntVec3 => gl::Uniform3iv(self.attrib_location as i32, 1, field_ptr as *const _),
				IntVec4 => gl::Uniform4iv(self.attrib_location as i32, 1, field_ptr as *const _),
				Mat2 => gl::UniformMatrix2fv(self.attrib_location as i32, 1, 0, field_ptr as *const _),
				Mat3 => gl::UniformMatrix3fv(self.attrib_location as i32, 1, 0, field_ptr as *const _),
				Mat4 => gl::UniformMatrix4fv(self.attrib_location as i32, 1, 0, field_ptr as *const _),
				Sampler2D => {
					gl::Uniform1i(self.attrib_location as i32, id as i32);
					let ref tex_res_ref = *(field_ptr as *const Arc<TextureHandle>);
					if let Some(tex_res) = tex_res_ref.as_any().downcast_ref::<GlTextureHandle>() {
						tex_res.bind_uniform(id);
					}
				}
			}
		}
	}
}

/*struct UniformFieldInfo {
	attrib_location: GLuint,
}*/

struct ResolvedShaderProgram {
	program: GlProgram,
	vertex_struct_size: GLsizei,
	vertex_fields: Vec<VertexFieldInfo>,
	uniform_item_locals: GLuint,
	//uniform_locals: GLuint,
	uniform_locals_fields: Vec<VertexFieldInfo>,
	//uniform_struct_size: GLsizei,
	//uniform_fields: Vec<UniformFieldInfo>,
}

impl ResolvedShaderProgram {
	fn new(vert_shader: GlShader, frag_shader: GlShader, shader: &Shader) -> Result<ResolvedShaderProgram, String> {
		
		let vertex_struct_size = shader.vertex_byte_count() as GLsizei;
		let mut vertex_fields = vec![];
		let mut uniform_locals_fields = vec![];
		let mut bindings = vec![];
		let mut current_binding = 0;
		
		shader.visit_vertex_structure(&mut |name: &str, offset: usize, shader_value_type: ShaderValueType| {
			if let Some((size, field_type)) = vertex_size_and_type(&shader_value_type) {
				//let attrib_location = unsafe { gl::GetAttribLocation(program.handle, CString) };
				
				vertex_fields.push(VertexFieldInfo {
					attrib_location: current_binding as GLuint,
					size: size as GLint,
					shader_value_type,
					field_type,
					offset: offset as *const c_void,
				});
				
				bindings.push((CString::new(name).unwrap(), current_binding));
				
				current_binding += 1;
			}
		});
		
		shader.visit_uniform_structure(&mut |name: &str, offset: usize, shader_value_type: ShaderValueType| {
			if let Some((size, field_type)) = vertex_size_and_type(&shader_value_type) {
				uniform_locals_fields.push(VertexFieldInfo {
					attrib_location: current_binding as GLuint,
					size: size as GLint,
					shader_value_type,
					field_type,
					offset: offset as *const c_void,
				});
				
				bindings.push((CString::new(name).unwrap(), current_binding));
				
				current_binding += 1;
			}
		});
		
		let program = GlProgram::link(vert_shader, frag_shader, &bindings)?;
		program.use_program();
		
		let uniform_item_locals = unsafe { gl::GetUniformBlockIndex(program.handle, CString::new("ItemLocals").unwrap().as_ptr()) };
		if uniform_item_locals == gl::INVALID_INDEX {
			return Err("Expected ItemLocals uniform block in shader".into());
		}
		/*let uniform_locals = unsafe { gl::GetUniformBlockIndex(program.handle, CString::new("Locals").unwrap().as_ptr()) };
		if uniform_locals == gl::INVALID_INDEX {
			return Err("Expected Locals uniform block in shader".into());
		}*/
		
		/*{
			let visit_fn = &mut |name: &str, offset: usize, shader_value_type: ShaderValueType| {
				if let Some((size, field_type)) = vertex_size_and_type(shader_value_type) {
					let attrib_location = unsafe { gl::GetAttribLocation(program.handle, CString::new(name).unwrap().as_ptr()) };
					
					vertex_fields.push(UniformFieldInfo {
						attrib_location: attrib_location as GLuint,
						size: size as GLint,
						field_type,
						offset: offset as *const c_void,
					});
				}
			};
			
			shader.visit_uniform_structure(visit_fn);
			ItemLocals::visit_structure(visit_fn);
		}*/
		
		Ok(ResolvedShaderProgram {
			program,
			vertex_struct_size,
			vertex_fields,
			uniform_item_locals,
			//uniform_locals,
			uniform_locals_fields,
		})
	}
	
	fn use_as_vertex_buffer(&self) {
		for field in &self.vertex_fields {
			unsafe {
				gl::VertexAttribPointer(
					field.attrib_location as GLuint,
					field.size,
					field.field_type,
					gl::FALSE as GLboolean,
					self.vertex_struct_size,
					field.offset,
				);
				
				gl::EnableVertexAttribArray(field.attrib_location as GLuint);
			}
		}
	}
}

/// This struct caches Pipeline State Objects created from Shaders.
struct ShaderCache {
	/// This is a mapping from shader hashes to their PSO.
	cache: HashMap<u64, Rc<ResolvedShaderProgram>>,
}

impl ShaderCache {
	/// Create a new empty cache.
	fn new() -> ShaderCache {
		ShaderCache {
			cache: HashMap::new(),
		}
	}

	fn get_shader_program(&mut self, shader: &Shader) -> Result<Rc<ResolvedShaderProgram>, String> {
		let opt_hash = shader.hash();
		if let Some(hash) = opt_hash {
			if let Some(ref program) = self.cache.get(&hash) {
				return Ok((*program).clone());
			}
		}
		
		let prelude = "
			#version 150 core
			
			layout (std140) uniform ItemLocals {
				mat4 uTransform;
				vec2 uPos;
				vec2 uSize;
			};
		";
		
		let vert_prefix = "#define VERT 1\n";
		let frag_prefix = "#define FRAG 1\n";
		
		let post_prelude = "#line 1\n";
		
		let vert_source = String::new() + prelude + vert_prefix + post_prelude + shader.code();
		let frag_source = String::new() + prelude + frag_prefix + post_prelude + shader.code();
		
		let vert_shader = GlShader::compile(&vert_source, gl::VERTEX_SHADER)?;
		let frag_shader = GlShader::compile(&frag_source, gl::FRAGMENT_SHADER)?;
		//let program = GlProgram::link(vert_shader, frag_shader)?;
		let resolved_program = ResolvedShaderProgram::new(vert_shader, frag_shader, shader)?;
		
		let rc_program = Rc::new(resolved_program);
		if let Some(hash) = opt_hash {
			self.cache.insert(hash, rc_program.clone());
		}
		Ok(rc_program)
		
		//println!("{:?}", new_shader_program.uniforms());
		
		//new_shader_program
	}
}

#[derive(Debug)]
struct GlTextureHandle {
	handle: GLuint,
}

/// Returns (internal_format, format, type).
fn image_format_to_gl_format(image_format: image_data::ImageFormat) -> Option<(GLuint, GLenum, GLenum)> {
	use crate::image_data::ImageFormat::*;
	match image_format {
		R8 => Some((gl::R8, gl::RED, gl::UNSIGNED_BYTE)),
		RGB888 => Some((gl::RGB8, gl::RGB, gl::UNSIGNED_BYTE)),
		RGBA8888 => Some((gl::RGBA8, gl::RGBA, gl::UNSIGNED_BYTE)),
	}
}

impl GlTextureHandle {
	fn new(image: &image_data::ImageData) -> Option<GlTextureHandle> {
		if let Some((internal_format, format, ty)) = image_format_to_gl_format(image.format()) {
			let mut handle = 0;
			unsafe {
				gl::GenTextures(1, &mut handle);
			}
			let tex = GlTextureHandle {
				handle,
			};
			tex.bind();
			
			let pixel_data = image.data();
			
			let size = image.size();
			unsafe {
				gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
			
				gl::TexImage2D(
					gl::TEXTURE_2D,
					0,
					internal_format as GLint,
					size.x as i32,
					size.y as i32,
					0,
					format,
					ty,
					pixel_data.as_ptr() as *const _);
					
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			}
			
			Some(tex)
		} else {
			None
		}
	}
	
	fn bind_uniform(&self, id: u32) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + id);
		}
		self.bind()
	}
	
	fn bind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.handle);
		}
	}
}

impl Drop for GlTextureHandle {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteTextures(1, &self.handle);
		}
	}
}

impl TextureHandle for GlTextureHandle {
	fn as_any(&self) -> &Any {
		self
	}
	
	fn blit(&self, image: &image_data::ImageData, rect: Rect<isize>) -> bool {
		if let Some((_internal_format, format, ty)) = image_format_to_gl_format(image.format()) {
			self.bind();
			
			let pixel_data = image.data();
			
			let _size = image.size();
			unsafe {
				gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
			
				gl::TexSubImage2D(
					gl::TEXTURE_2D,
					0,
					rect.left() as i32,
					rect.top() as i32,
					rect.width() as i32,
					rect.height() as i32,
					format,
					ty,
					pixel_data.as_ptr() as *const _);
					
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
				gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
			}
			
			true
		} else {
			false
		}
	}
}

pub struct GlTextureResource;

impl TextureResourceImpl for GlTextureResource {
	fn make_texture(&self, image_data: &image_data::ImageData) -> Arc<TextureHandle> {
		Arc::new(GlTextureHandle::new(image_data).unwrap())
	}
}

pub struct DisplayCache {
	item_cache: ItemCache,
	shader_cache: ShaderCache,
	message_sender: mpsc::Sender<item::ItemUpdateMessage>,
	message_receiver: mpsc::Receiver<item::ItemUpdateMessage>,
	mouse_hover_id: Option<usize>,
	mouse_press_id: Option<usize>,
	pressed_mouse_buttons: MouseButtons,
	animating_items: HashSet<usize>,
	resource_manager: GraphicResourceManager,
}

impl DisplayCache {
	pub fn new() -> DisplayCache {
		let (message_sender, message_receiver) = mpsc::channel();
		
		let mut resource_manager = GraphicResourceManager::new();
		
		resource_manager.register_resource(Box::new(TextureResource::new(Box::new(GlTextureResource))));
		
		DisplayCache {
			item_cache: ItemCache::new(),
			shader_cache: ShaderCache::new(),
			message_sender, message_receiver,
			mouse_hover_id: None,
			mouse_press_id: None,
			pressed_mouse_buttons: MouseButtons::None,
			animating_items: HashSet::new(),
			resource_manager,
		}
	}
	
	pub fn register_resource(&mut self, resource: Box<GraphicResource>) -> Result<(), Box<GraphicResource>> {
		self.resource_manager.register_resource(resource)
	}
	
	pub fn draw(&mut self, transform: &cgmath::Matrix4<f32>) {
		self.item_cache.draw(transform, &mut self.resource_manager, &mut self.shader_cache);
	}

	pub fn process_item(&mut self,
		item: &Item,
		global_top_left: Vec2f,
		parent_cache_entry: Option<usize>,
	) {
		let global_item_pos = global_top_left + item.get_item().pos;
		
		let item_id: usize;
		
		{
			{
				let mut item_cache_entry: RefMut<ItemCacheEntry>;
				
				let mut opt_internal = item.get_item().internal.borrow_mut();
				if opt_internal.is_some() {
					let internal = opt_internal.as_ref().unwrap();
					item_id = internal.id;
					item_cache_entry = self.item_cache.alloc.get_mut(internal.id).borrow_mut();
					
					item_cache_entry.parent = parent_cache_entry;
					item_cache_entry.pos = item.get_item().pos;
					item_cache_entry.global_pos = global_item_pos;
					item_cache_entry.size = item.get_item().size;
				} else {
					let allocation = self.item_cache.alloc.allocate(RefCell::new(ItemCacheEntry {
						parent: parent_cache_entry,
						apply_handle: None,
						pos: item.get_item().pos,
						global_pos: global_item_pos,
						size: global_item_pos,
						geometry: None,
						vertex_array: None,
						vertex_buffer: None,
						//uniform_buffer: None,
						index_buffer: None,
						shader_program: None,
						is_mouse_area: item.get_item().mouse_data.is_some(),
					}));
					
					item_cache_entry = allocation.0.borrow_mut();
					item_id = allocation.1;
					
					// Record the allocated ItemCacheEntry ID in the Item itself.
					*opt_internal = Some(item::ItemDataInternal {
						id: item_id,
						message_sender: self.message_sender.clone(),
					});
				}
				
				// The update_geometry method consumes the original geometry, so this pulls the
				// value of the geometry out of the cache entry so it can be given to
				// update_geometry without borrowing item_cache_entry.
				let original_geometry = std::mem::replace(&mut item_cache_entry.geometry, None);
				let new_geometry = item.update_geometry(original_geometry, &mut self.resource_manager);
				
				//glEnableVertexAttribArray
				
				// If an item returns no geometry, it has nothing to be rendered.
				/*if let Some(ref geometry) = new_geometry {
					let shader = geometry.shader();
					
					let shader_program = self.shader_cache.get_shader_program(shader).unwrap();
					
					let vertex_array = GlVertexArray::new();
					vertex_array.bind();
					
					let vertex_buffer = GlArrayBuffer::new(geometry.vertex_data(), gl::ARRAY_BUFFER, gl::STATIC_DRAW);
					
					//let uniform_buffer = GlArrayBuffer::new(geometry.uniform_data(), gl::ARRAY_BUFFER);
					
					let index_buffer = GlIndexBuffer::new(geometry.indices());
					
					item_cache_entry.vertex_array = Some(vertex_array);
					item_cache_entry.shader_program = Some(shader_program);
					item_cache_entry.vertex_buffer = Some(vertex_buffer);
					//item_cache_entry.uniform_buffer = Some(uniform_buffer);
					item_cache_entry.index_buffer = Some(index_buffer);
					
					/*let index_buffer = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, geometry.indices()).unwrap();
					item_cache_entry.index_buffer = Some(index_buffer);
					
					let vertex_buffer = glium::buffer::Buffer::empty_unsized(display,
						glium::buffer::BufferType::ArrayBuffer,
						geometry.vertex_data().len(),
						glium::buffer::BufferMode::Persistent).unwrap();
						
					item_cache_entry.vertex_buffer = Some(vertex_buffer);
					
					if item_cache_entry.shader_program.is_none() {
						let shader_program = self.shader_cache.get_shader_program(display, shader);
						item_cache_entry.shader_program = Some(shader_program);
					}*/
				}*/
				
				item_cache_entry.geometry = new_geometry;
				item_cache_entry.process_geometry(&mut self.shader_cache);
			}
			
			if parent_cache_entry.is_none() {
				self.item_cache.main_item = Some(item_id);
			}
		}
		
		item.apply_to_children(&mut |items| {
			for item in items {
				self.process_item(*item, global_item_pos, Some(item_id));
			}
		});
	}
	
	pub fn process_messages(&mut self) {
		for message in self.message_receiver.try_iter() {
			use crate::ItemUpdateMessage::*;
			
			match message {
				MarkDamaged{id: _} => {
				
				}
				UpdateApplyHandle{id, apply_handle} => {
					let mut item_cache_entry = self.item_cache.alloc.get_mut(id).borrow_mut();
					item_cache_entry.apply_handle = Some(apply_handle);
				}
				ChildrenAdded{id: _, indices: _} => {
				
				}
				ChildrenRemoved{id: _, indices: _} => {
				
				}
				AnimationStarted{id} => {
					self.animating_items.insert(id);
				}
				AnimationStopped{id} => {
					self.animating_items.remove(&id);
				}
			}
		}
	}
	
	pub fn is_animating(&self) -> bool {
		!self.animating_items.is_empty()
	}
	
	pub fn send_animation_signals(&mut self, time_step: f64) {
		for &item_id in &self.animating_items {
			let item_cache_entry = self.item_cache.alloc.get_mut(item_id);
			if let Some(ref apply_handle) = item_cache_entry.borrow().apply_handle {
				apply_handle.invoke(ItemDataChange::animation_frame(JustSignalChange(time_step)));
			}
		}
	}
	
	pub fn process_mouse_pos(&mut self, pos: Vec2f) {
		let opt_hover_id = self.item_cache.hover_item(pos);
		if self.mouse_hover_id != opt_hover_id {
			if let Some(mouse_hover_id) = self.mouse_hover_id {
				let old_hover_item_cache_entry = self.item_cache.alloc.get(mouse_hover_id);
				if let Some(ref apply_handle) = old_hover_item_cache_entry.borrow().apply_handle {
					apply_handle.invoke(ItemDataChange::mouse_data(OptionChange::Change(MouseDataChange::contains_mouse(ValueChange(false)))));
				}
			}
			
			if let Some(hover_id) = opt_hover_id {
				let hover_item_cache_entry = self.item_cache.alloc.get(hover_id);
				if let Some(ref apply_handle) = hover_item_cache_entry.borrow().apply_handle {
					apply_handle.invoke(ItemDataChange::mouse_data(OptionChange::Change(MouseDataChange::contains_mouse(ValueChange(true)))));
				}
			}
			
			self.mouse_hover_id = opt_hover_id;
		}
	}
	
	pub fn process_mouse_down(&mut self, button: MouseButton) {
		if self.pressed_mouse_buttons == MouseButtons::None {
			self.mouse_press_id = self.mouse_hover_id;
		}
		
		self.pressed_mouse_buttons.set_button(button, true);
		
		if let Some(mouse_press_id) = self.mouse_press_id {
			let pressed_item_cache_entry = self.item_cache.alloc.get(mouse_press_id);
			if let Some(ref apply_handle) = pressed_item_cache_entry.borrow().apply_handle {
				apply_handle.invoke(ItemDataChange::mouse_data(OptionChange::Change(MouseDataChange::pressed(ValueChange(self.pressed_mouse_buttons)))));
			}
		}
	}
	
	pub fn process_mouse_up(&mut self, button: MouseButton) {
		self.pressed_mouse_buttons.set_button(button, false);
		
		if let Some(mouse_press_id) = self.mouse_press_id {
			let pressed_item_cache_entry = self.item_cache.alloc.get(mouse_press_id);
			if let Some(ref apply_handle) = pressed_item_cache_entry.borrow().apply_handle {
				apply_handle.invoke(ItemDataChange::mouse_data(OptionChange::Change(MouseDataChange::pressed(ValueChange(self.pressed_mouse_buttons)))));
			}
		}
		
		if self.pressed_mouse_buttons == MouseButtons::None {
			self.mouse_press_id = None;
		}
	}
}
