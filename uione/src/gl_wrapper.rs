use crate::cast_slice::cast_slice;

use gl;
use gl::types::{GLuint, GLint, GLchar, GLenum, GLsizeiptr, GLsizei};
use std;
use std::ffi::CString;
use std::error::Error;

pub struct GlShader {
	pub handle: GLuint,
}

impl GlShader {
	pub fn new(shader_type: GLuint) -> GlShader {
		unsafe {
			GlShader { handle: gl::CreateShader(shader_type) }
		}
	}
	
	pub fn compile(code: &str, shader_type: GLenum) -> Result<GlShader, String> {
		let shader;
		unsafe {
			shader = GlShader::new(shader_type);
			let c_code = CString::new(code.as_bytes()).map_err(|e| e.description().to_string())?;
			gl::ShaderSource(shader.handle, 1, &c_code.as_ptr(), std::ptr::null());

			gl::CompileShader(shader.handle);
			let mut status = gl::FALSE as GLint;
			gl::GetShaderiv(shader.handle, gl::COMPILE_STATUS, &mut status);

			if status != gl::TRUE as GLint {
				// Compilation failed.
				let mut info_len = 0;
				gl::GetShaderiv(shader.handle, gl::INFO_LOG_LENGTH, &mut info_len);
				// There is always a null character at the end of the info string, this ignores it:
				if info_len > 0 {
					info_len -= 1;
				}
				
				let mut buf = Vec::with_capacity(info_len as usize);
				buf.set_len(info_len as usize);
				gl::GetShaderInfoLog(
					shader.handle,
					info_len,
					std::ptr::null_mut(),
					buf.as_mut_ptr() as *mut GLchar,
				);
				
				return Err(String::from_utf8(buf).ok()
						.expect("ShaderInfoLog is not a valid UTF8 string"));
			}
		}
		Ok(shader)
	}
}

impl Drop for GlShader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.handle);
		}
	}
}

pub struct GlProgram {
	pub vert_shader: GlShader,
	pub frag_shader: GlShader,
	pub handle: GLuint,
}

impl GlProgram {
	pub fn link(vert_shader: GlShader, frag_shader: GlShader, attrib_bindings: &[(CString, GLuint)]) -> Result<GlProgram, String> {
		let program_handle = unsafe { gl::CreateProgram() };
		let program = GlProgram {
			vert_shader, frag_shader,
			handle: program_handle,
		};
		
		unsafe {
			gl::AttachShader(program.handle, program.vert_shader.handle);
			gl::AttachShader(program.handle, program.frag_shader.handle);
			
			for binding in attrib_bindings {
				gl::BindAttribLocation(program.handle, binding.1, binding.0.as_ptr());
			}
			
			gl::LinkProgram(program.handle);
			
			let mut status = gl::FALSE as GLint;
			gl::GetProgramiv(program.handle, gl::LINK_STATUS, &mut status);

			if status != gl::TRUE as GLint {
				// Compilation failed.
				let mut info_len = 0;
				gl::GetProgramiv(program.handle, gl::INFO_LOG_LENGTH, &mut info_len);
				// There is always a null character at the end of the info string, this ignores it:
				if info_len > 0 {
					info_len -= 1;
				}
				
				let mut buf = Vec::with_capacity(info_len as usize);
				buf.set_len(info_len as usize);
				gl::GetProgramInfoLog(
					program.handle,
					info_len,
					std::ptr::null_mut(),
					buf.as_mut_ptr() as *mut GLchar,
				);
				
				return Err(String::from_utf8(buf).ok()
						.expect("ProgramInfoLog is not a valid UTF8 string"));
			}
		}
		
		Ok(program)
	}
	
	pub fn use_program(&self) {
		unsafe {
			gl::UseProgram(self.handle);
		}
	}
}

impl Drop for GlProgram {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.handle);
		}
	}
}

pub struct GlVertexArray {
	pub handle: GLuint,
}

impl GlVertexArray {
	pub fn new() -> GlVertexArray {
		let mut handle = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut handle);
		}
		GlVertexArray {
			handle
		}
	}
	
	pub fn bind(&self) {
		unsafe {
			gl::BindVertexArray(self.handle);
		}
	}
}

impl Drop for GlVertexArray {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteVertexArrays(1, &self.handle);
		}
	}
}

pub struct GlArrayBuffer {
	pub handle: GLuint,
	pub target: GLenum,
}

impl GlArrayBuffer {
	pub fn new(data: &[u8], target: GLenum, usage: GLenum) -> GlArrayBuffer {
		let mut handle = 0;
		unsafe {
			gl::GenBuffers(1, &mut handle);
		}
		let buffer = GlArrayBuffer {
			handle,
			target,
		};
		buffer.bind();
		
		unsafe {
			gl::BufferData(
				buffer.target,
				data.len() as GLsizeiptr,
				std::mem::transmute(data.as_ptr()),
				usage,
			);
		}
		
		buffer
	}
	
	pub fn bind(&self) {
		unsafe {
			gl::BindBuffer(self.target, self.handle);
		}
	}
}

impl Drop for GlArrayBuffer {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.handle);
		}
	}
}

pub struct GlIndexBuffer {
	pub array_buffer: GlArrayBuffer,
	pub count: GLsizei,
}

impl GlIndexBuffer {
	pub fn new(indices: &[u32]) -> GlIndexBuffer {
		let raw_indices = unsafe { cast_slice(indices) };
		let array_buffer = GlArrayBuffer::new(raw_indices, gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
		GlIndexBuffer {
			array_buffer,
			count: indices.len() as GLsizei,
		}
	}
	
	pub fn draw(&self) {
		self.array_buffer.bind();
		unsafe {
			gl::DrawElements(gl::TRIANGLES, self.count, gl::UNSIGNED_INT, std::ptr::null());
		}
	}
}
