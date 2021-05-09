use crate::shader::*;
use crate::cast_slice::cast_slice;
use crate::resources::GraphicResourceManager;

use crate::vec2::*;

use std;
use std::any::Any;
use bitflags::bitflags;

/*/// Represents the type of update that is expected to be applied to the geometry when an Item's
/// update_geometry function is called.
pub enum GeometryUpdateType {
	/// Everything was damaged, so make sure everything is up-to-date.
	Everything,
	/// The geometry is about to be drawn and any last moment updates should be applied.
	PreDraw,
}*/

bitflags! {
	pub struct GeometryDamageFlags: u8 {
		const NONE         = 0b00000000;
		const VERTEX_DATA  = 0b00000001;
		const UNIFORM_DATA = 0b00000010;
		const INDICES      = 0b00000100;
		const SIZE         = 0b00001000;
		const EVERYTHING   =
			Self::VERTEX_DATA.bits |
			Self::UNIFORM_DATA.bits |
			Self::INDICES.bits |
			Self::SIZE.bits;
	}
}

pub unsafe trait Geometry: std::fmt::Debug + Send + Sync {
	/// The actual vertex data.
	fn vertex_data(&self) -> &[u8];
	
	/// The actual uniform data.
	fn uniform_data(&self) -> &[u8];
	
	/// The a list of vertex indices specifying in which order to use the
	/// vertices.
	fn indices(&self) -> &Vec<u32>;
	
	fn size(&self) -> Vec2f;
	
	/// The shader this `Geometry` uses. This is also used to determine the layout
	/// of the vertex and uniform data.
	fn shader(&self) -> &Shader;
	
	//fn query_vertex_name(&self, name: &str) -> Option<gfx::pso::buffer::Element<gfx::format::Format>>;
	
	/// This can be used to call custom code when the scene graph is about to draw this Geometry.
	fn on_draw(&mut self, _resource_manager: &mut GraphicResourceManager) {}
	
	/// Call this after the `damage_flags` have been read and dealt with to reset them to their
	/// default state (eg. `GeometryDamageFlags::NONE`).
	fn reset_damage_flags(&mut self) {}
	
	/// Get the set of things in the geometry that actually changed.
	fn damage_flags(&self) -> GeometryDamageFlags {
		GeometryDamageFlags::EVERYTHING
	}
	
	fn as_any(&self) -> &Any;
	fn as_any_mut(&mut self) -> &mut Any;
}

#[derive(Debug)]
pub struct BasicGeometry<ShaderType: TypedShader> {
	pub vertices: Vec<ShaderType::VertexType>,
	pub indices: Vec<u32>,
	pub size: Vec2f,
	pub uniform: ShaderType::UniformType,
	pub shader: Box<Shader>,
	pub damage_flags: GeometryDamageFlags,
}

unsafe impl<ShaderType: TypedShader + 'static> Geometry for BasicGeometry<ShaderType> {
	fn vertex_data(&self) -> &[u8] {
		unsafe { cast_slice(&self.vertices) }
	}
	
	fn indices(&self) -> &Vec<u32> {
		&self.indices
	}
	
	fn size(&self) -> Vec2f {
		self.size
	}
	
	fn uniform_data(&self) -> &[u8] {
		unsafe {
			std::slice::from_raw_parts(
				std::mem::transmute(&self.uniform),
				std::mem::size_of::<ShaderType::UniformType>()
			)
		}
	}
	
	fn shader(&self) -> &Shader {
		self.shader.as_ref()
	}
	
	fn reset_damage_flags(&mut self) {
		self.damage_flags = GeometryDamageFlags::NONE;
	}
	
	fn damage_flags(&self) -> GeometryDamageFlags {
		self.damage_flags
	}
	
	fn as_any(&self) -> &Any { self }
	fn as_any_mut(&mut self) -> &mut Any { self }
}

pub struct BasicFnGeometry<ShaderType: TypedShader, ArgumentType> {
	pub basic_geometry: BasicGeometry<ShaderType>,
	pub on_draw_arg: ArgumentType,
	pub on_draw_fn: fn(arg: &mut ArgumentType, geometry: &mut BasicGeometry<ShaderType>, resource_manager: &mut GraphicResourceManager),
}

unsafe impl<ShaderType: TypedShader + 'static, ArgumentType: Sync + Send + 'static> Geometry for BasicFnGeometry<ShaderType, ArgumentType> {
	fn vertex_data(&self) -> &[u8] {
		self.basic_geometry.vertex_data()
	}
	
	fn indices(&self) -> &Vec<u32> {
		self.basic_geometry.indices()
	}
	
	fn size(&self) -> Vec2f {
		self.basic_geometry.size()
	}
	
	fn uniform_data(&self) -> &[u8] {
		self.basic_geometry.uniform_data()
	}
	
	fn shader(&self) -> &Shader {
		self.basic_geometry.shader()
	}
	
	fn reset_damage_flags(&mut self) {
		self.basic_geometry.reset_damage_flags();
	}
	
	fn damage_flags(&self) -> GeometryDamageFlags {
		self.basic_geometry.damage_flags()
	}
	
	fn on_draw(&mut self, resource_manager: &mut GraphicResourceManager) {
		(self.on_draw_fn)(&mut self.on_draw_arg, &mut self.basic_geometry, resource_manager);
	}
	
	fn as_any(&self) -> &Any { self }
	fn as_any_mut(&mut self) -> &mut Any { self }
}

impl<ShaderType: TypedShader + 'static, ArgumentType> std::fmt::Debug for BasicFnGeometry<ShaderType, ArgumentType> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "BasicFnGeometry: {:?}", self.basic_geometry)
	}
}

/*pub type ColourFormat = gfx::format::Srgba8;

gfx_defines! {
	vertex BasicVert {
		pos: [f32; 2] = "vPos",
		colour: [f32; 4] = "vColour",
	}

	constant BasicLocals {
		transform: [[f32; 4]; 4] = "uTransform",
		pos: [f32; 2] = "uPos",
		size: [f32; 2] = "uSize",
	}
	
	pipeline BasicPipe {
		vbuf: gfx::VertexBuffer<BasicVert> = (),
		locals: gfx::ConstantBuffer<BasicLocals> = "Locals",
		out: gfx::RenderTarget<ColourFormat> = "Target0",
	}
}*/
