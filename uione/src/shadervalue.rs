use crate::texture_resource::TextureHandle;

use std::sync::Arc;

pub unsafe trait ConvertsToShaderValue {
	fn shader_value_type() -> ShaderValueType;
	fn shader_value(&self) -> ShaderValue;
}

macro_rules! impl_shader_value_types {
	(
		$($shader_type:ident($rust_type:ty, $shader_name:expr),)*
	) => {
		#[derive(Debug, Clone)]
		pub enum ShaderValueType {
			$($shader_type,)*
		}
		
		impl ShaderValueType {
			pub fn glsl_name(&self) -> &'static str {
				match self {
					$(ShaderValueType::$shader_type => $shader_name,)*
				}
			}
		}
		
		#[derive(Debug)]
		pub enum ShaderValue {
			$($shader_type($rust_type),)*
		}
		
		$(
			unsafe impl ConvertsToShaderValue for $rust_type {
				fn shader_value_type() -> ShaderValueType {
					$crate::shadervalue::ShaderValueType::$shader_type
				}
				
				fn shader_value(&self) -> ShaderValue {
					$crate::shadervalue::ShaderValue::$shader_type(self.clone())
				}
			}
		)*
	}
}

impl_shader_value_types! {
	//Bool(bool, "bool"),
	Int(i32, "int"),
	UInt(u32, "uint"),
	Float(f32, "float"),
	//Double(f64, "double"),
	Vec2([f32; 2], "vec2"),
	Vec3([f32; 3], "vec3"),
	Vec4([f32; 4], "vec4"),
	IntVec2([i32; 2], "ivec2"),
	IntVec3([i32; 3], "ivec3"),
	IntVec4([i32; 4], "ivec4"),
	Mat2([[f32; 2]; 2], "mat2"),
	Mat3([[f32; 3]; 3], "mat3"),
	Mat4([[f32; 4]; 4], "mat4"),
	Sampler2D(Arc<TextureHandle>, "sampler2D"),
}
