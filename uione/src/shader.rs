use crate::shadervalue::*;

use std;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[macro_export] macro_rules! thing_and_stuff {
	() => {}
}

#[macro_export] macro_rules! uione_uniform_struct {
	(
		$(#[$attr:meta])*
		struct $struct_name:ident {
			$(
				$field_name:ident : $field_type:ty = $shader_name:expr,
			)*
		}
	) => {
		$(#[$attr])*
		#[derive(Debug, Clone)]
		#[repr(C)]
		pub struct $struct_name {
			$(
				$field_name: $field_type,
			)*
		}
		
		unsafe impl $crate::shader::UniformStruct for $struct_name {
			fn visit_structure(func: &mut FnMut(/*name:*/ &str, /*offset:*/ usize, /*shader_value_type:*/ $crate::shadervalue::ShaderValueType)) {
				let dummy: $struct_name = unsafe { ::std::mem::MaybeUninit::uninit().assume_init() };
				{
					let dummy_ref = &dummy;
					$(
						{
							let field_ref = &dummy.$field_name;
							let offset = (field_ref as *const _ as usize) - (dummy_ref as *const _ as usize);
							
							func($shader_name, offset, <$field_type as $crate::shadervalue::ConvertsToShaderValue>::shader_value_type());
						}
					)*
				}
				::std::mem::forget(dummy);
			}
			
			fn visit_values(&self, func: &mut FnMut(&str, $crate::shadervalue::ShaderValue)) {
				use $crate::shadervalue::ConvertsToShaderValue;
				$(
					//println!("{}: {:?}", $shader_name, stringify!($field_name));
					func($shader_name, self.$field_name.shader_value());
				)*
			}
		}
	};
	
	(
		$(#[$attr:meta])*
		struct $struct_name:ident {
			$(
				$field_name:ident : $field_type:ty = name:expr
			),*
		}
	) => {
		uione_uniform_struct! {
			$(#[$attr])*
			struct $struct_name {
				$(
					$field_name : $field_type = name:expr,
				)*
			}
		}
	};
}

pub unsafe trait UniformStruct {
	fn visit_structure(func: &mut FnMut(/*name:*/ &str, /*offset:*/ usize, /*shader_value_type:*/ ShaderValueType));
	fn visit_values(&self, func: &mut FnMut(&str, ShaderValue));
}

/*shader_struct! {
	struct Vert {
		pos: [f32; 2] = "pos",
		colour: [f32; 4] = "colour",
		border: i32 = "border",
	}
}

shader_struct! {
	struct Locals {
		gradient: f32 = "uGradient",
	}
}*/

pub trait Shader: std::fmt::Debug + Send + Sync {
	/// Get the unique hash of this shader for use with optimising matching
	/// shaders in the scene graph. If the hash is None, then it will not
	/// automatically match any other shaders.
	fn hash(&self) -> Option<u64> {
		None
	}
	
	fn code(&self) -> &str;
	
	/// Number of bytes per vertex.
	fn vertex_byte_count(&self) -> usize;
	fn uniform_byte_count(&self) -> usize;
	
	fn visit_vertex_structure(&self, func: &mut FnMut(/*name:*/ &str, /*offset:*/ usize, /*shader_value_type:*/ ShaderValueType));
	
	fn visit_uniform_structure(&self, func: &mut FnMut(/*name:*/ &str, /*offset:*/ usize, /*shader_value_type:*/ ShaderValueType));
	
	//unsafe fn visit_uniform_values<'u>(&'u self, data: &[u8], func: &mut FnMut(&'u str, glium::uniforms::UniformValue<'u>));
	
	//fn vertex_format(&self) -> glium::vertex::VertexFormat;
	
	//fn vertex_init<'a>(&'a self) -> (&'a [(&'a str, gfx::pso::buffer::Element<gfx::format::Format>)], u8, u8);
	
	//fn query_vertex_name(&self, name: &str) -> Option<gfx::pso::buffer::Element<gfx::format::Format>>;
	//fn query_uniform_name(&self, name: &str) -> Option<gfx::pso::buffer::Element<gfx::shade::ConstFormat>>;
}

pub trait TypedShader: Shader {
	type VertexType: UniformStruct + std::fmt::Debug + Send + Sync;
	type UniformType: UniformStruct + std::fmt::Debug + Send + Sync;
}

#[derive(Debug)]
pub struct BasicShader<Vertex, Uniform> {
	code: String,
	hash: u64,
	vertex_phantom: std::marker::PhantomData<Vertex>,
	uniform_phantom: std::marker::PhantomData<Uniform>,
}

impl<Vertex, Uniform> BasicShader<Vertex, Uniform> {
	pub fn new(code: String) -> BasicShader<Vertex, Uniform> {
		let mut hasher = DefaultHasher::new();
		code.hash(&mut hasher);
		code.hash(&mut hasher);
		BasicShader {
			code,
			hash: hasher.finish(),
			vertex_phantom: std::marker::PhantomData,
			uniform_phantom: std::marker::PhantomData,
		}
	}
}

impl<VertexType, UniformType> Shader for BasicShader<VertexType, UniformType> where
	VertexType: UniformStruct + std::fmt::Debug + Send + Sync,
	UniformType: UniformStruct + std::fmt::Debug + Send + Sync,
{
	fn hash(&self) -> Option<u64> {
		Some(self.hash)
	}
	
	fn code(&self) -> &str {
		&self.code
	}
	
	fn vertex_byte_count(&self) -> usize {
		std::mem::size_of::<VertexType>()
	}
	
	fn uniform_byte_count(&self) -> usize {
		std::mem::size_of::<UniformType>()
	}
	
	fn visit_vertex_structure(&self, func: &mut FnMut(/*name:*/ &str, /*offset:*/ usize, /*shader_value_type:*/ ShaderValueType)) {
		VertexType::visit_structure(func);
	}
	
	fn visit_uniform_structure(&self, func: &mut FnMut(/*name:*/ &str, /*offset:*/ usize, /*shader_value_type:*/ ShaderValueType)) {
		UniformType::visit_structure(func);
	}
	
	/*unsafe fn visit_uniform_values<'u>(&'u self, data: &[u8], func: &mut FnMut(&'u str, glium::uniforms::UniformValue<'u>)) {
		assert_eq!(data.len(), std::mem::size_of::<UniformType>());
		let struct_data: *const UniformType = std::mem::transmute(data.as_ptr());
		(*struct_data).visit_values(func);
	}
	
	fn vertex_format(&self) -> glium::vertex::VertexFormat {
		VertexType::build_bindings()
	}*/
	
	//fn vertex_init<'a>(&'a self) -> (&'a [(&'a str, gfx::pso::buffer::Element<gfx::format::Format>)], u8, u8) {
		
	//}
	
	/*fn query_vertex_name(&self, name: &str) -> Option<gfx::pso::buffer::Element<gfx::format::Format>> {
		VertexType::query(name)
	}
	
	fn query_uniform_name(&self, name: &str) -> Option<gfx::pso::buffer::Element<gfx::shade::ConstFormat>> {
		UniformType::query(name)
	}*/
}

impl<VertexType, UniformType> TypedShader for BasicShader<VertexType, UniformType> where
	VertexType: UniformStruct + std::fmt::Debug + Send + Sync,
	UniformType: UniformStruct + std::fmt::Debug + Send + Sync,
{
	type VertexType = VertexType;
	type UniformType = UniformType;
}

/*pub struct StaticShader {
	vert_code: &'static str,
	frag_code: &'static str,
	hash: u64,
}

impl StaticShader {
	pub fn new(vert_code: &'static str, frag_code: &'static str) -> StaticShader {
		let mut hasher = DefaultHasher::new();
		vert_code.hash(&mut hasher);
		frag_code.hash(&mut hasher);
		StaticShader { vert_code, frag_code, hash: hasher.finish() }
	}
}

impl Shader for StaticShader {
	fn hash(&self) -> Option<u64> {
		Some(self.hash)
	}
	
	fn vert_code(&self) -> &str {
		&self.vert_code
	}
	
	fn frag_code(&self) -> &str {
		&self.frag_code
	}
}*/
