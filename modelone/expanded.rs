#[allow(missing_docs)]
#[rustc_copy_clone_marker]
pub struct Vertex {
	pub pos: [f32; 2],
	pub colour: [f32; 4],
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::clone::Clone for Vertex {
	#[inline]
	fn clone(&self) -> Vertex {
		{
			let _: ::std::clone::AssertParamIsClone<[f32; 2]>;
			let _: ::std::clone::AssertParamIsClone<[f32; 4]>;
			*self
		}
	}
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::marker::Copy for Vertex { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::fmt::Debug for Vertex {
	fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
		-> ::std::fmt::Result {
		match *self {
			Vertex { pos: ref __self_0_0, colour: ref __self_0_1 } =>
			{
				let mut builder = __arg_0.debug_struct("Vertex");
				let _ = builder.field("pos", &&(*__self_0_0));
				let _ = builder.field("colour", &&(*__self_0_1));
				builder.finish()
			}
		}
	}
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::cmp::PartialEq for Vertex {
	#[inline]
	fn eq(&self, __arg_0: &Vertex) -> bool {
		match *__arg_0 {
			Vertex { pos: ref __self_1_0, colour: ref __self_1_1 } =>
			match *self {
				Vertex { pos: ref __self_0_0, colour: ref __self_0_1 }
				=>
				true && (*__self_0_0) == (*__self_1_0) &&
					(*__self_0_1) == (*__self_1_1),
			},
		}
	}
	#[inline]
	fn ne(&self, __arg_0: &Vertex) -> bool {
		match *__arg_0 {
			Vertex { pos: ref __self_1_0, colour: ref __self_1_1 } =>
			match *self {
				Vertex { pos: ref __self_0_0, colour: ref __self_0_1 }
				=>
				false || (*__self_0_0) != (*__self_1_0) ||
					(*__self_0_1) != (*__self_1_1),
			},
		}
	}
}
unsafe impl ::traits::Pod for Vertex { }
impl ::pso::buffer::Structure<::format::Format> for Vertex {
	fn query(name: &str)
		->
			::std::option::Option<::pso::buffer::Element<::format::Format>> {
		use std::mem::{size_of, transmute};
		use ::pso::buffer::{Element, ElemOffset};
		let tmp: &Vertex = unsafe { transmute(1usize) };
		let base = tmp as *const _ as usize;
		let (sub_name, big_offset) =
			{
				let mut split = name.split(|c| c == '[' || c == ']');
				let _ = split.next().unwrap();
				match split.next() {
					Some(s) => {
						let array_id: ElemOffset = s.parse().unwrap();
						let sub_name =
							match split.next() {
								Some(s) if s.starts_with('.') =>
								&s[1..],
								_ => name,
							};
						(sub_name,
							array_id *
								(size_of::<Vertex>() as ElemOffset))
					}
					None => (name, 0),
				}
			};
		match sub_name {
			"vPos" =>
			Some(Element{format:
								<[f32; 2] as
									::format::Formatted>::get_format(),
							offset:
								(((&tmp.pos as *const _ as usize) - base)
									as ElemOffset) + big_offset,}),
			"vColour" =>
			Some(Element{format:
								<[f32; 4] as
									::format::Formatted>::get_format(),
							offset:
								(((&tmp.colour as *const _ as usize) -
									base) as ElemOffset) +
									big_offset,}),
			_ => None,
		}
	}
}
#[allow(missing_docs)]
#[doc = r" Uniforms that apply to all Item shaders."]
#[rustc_copy_clone_marker]
pub struct ItemLocals {
	pub transform: [[f32; 4]; 4],
	pub pos: [f32; 2],
	pub size: [f32; 2],
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::clone::Clone for ItemLocals {
	#[inline]
	fn clone(&self) -> ItemLocals {
		{
			let _: ::std::clone::AssertParamIsClone<[[f32; 4]; 4]>;
			let _: ::std::clone::AssertParamIsClone<[f32; 2]>;
			let _: ::std::clone::AssertParamIsClone<[f32; 2]>;
			*self
		}
	}
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::marker::Copy for ItemLocals { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::fmt::Debug for ItemLocals {
	fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
		-> ::std::fmt::Result {
		match *self {
			ItemLocals {
			transform: ref __self_0_0,
			pos: ref __self_0_1,
			size: ref __self_0_2 } => {
				let mut builder = __arg_0.debug_struct("ItemLocals");
				let _ = builder.field("transform", &&(*__self_0_0));
				let _ = builder.field("pos", &&(*__self_0_1));
				let _ = builder.field("size", &&(*__self_0_2));
				builder.finish()
			}
		}
	}
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::cmp::PartialEq for ItemLocals {
	#[inline]
	fn eq(&self, __arg_0: &ItemLocals) -> bool {
		match *__arg_0 {
			ItemLocals {
			transform: ref __self_1_0,
			pos: ref __self_1_1,
			size: ref __self_1_2 } =>
			match *self {
				ItemLocals {
				transform: ref __self_0_0,
				pos: ref __self_0_1,
				size: ref __self_0_2 } =>
				true && (*__self_0_0) == (*__self_1_0) &&
					(*__self_0_1) == (*__self_1_1) &&
					(*__self_0_2) == (*__self_1_2),
			},
		}
	}
	#[inline]
	fn ne(&self, __arg_0: &ItemLocals) -> bool {
		match *__arg_0 {
			ItemLocals {
			transform: ref __self_1_0,
			pos: ref __self_1_1,
			size: ref __self_1_2 } =>
			match *self {
				ItemLocals {
				transform: ref __self_0_0,
				pos: ref __self_0_1,
				size: ref __self_0_2 } =>
				false || (*__self_0_0) != (*__self_1_0) ||
					(*__self_0_1) != (*__self_1_1) ||
					(*__self_0_2) != (*__self_1_2),
			},
		}
	}
}
unsafe impl ::traits::Pod for ItemLocals { }
impl ::pso::buffer::Structure<::shade::ConstFormat> for ItemLocals {
	fn query(name: &str)
		->
			::std::option::Option<::pso::buffer::Element<::shade::ConstFormat>> {
		use std::mem::{size_of, transmute};
		use ::pso::buffer::{Element, ElemOffset};
		let tmp: &ItemLocals = unsafe { transmute(1usize) };
		let base = tmp as *const _ as usize;
		let (sub_name, big_offset) =
			{
				let mut split = name.split(|c| c == '[' || c == ']');
				let _ = split.next().unwrap();
				match split.next() {
					Some(s) => {
						let array_id: ElemOffset = s.parse().unwrap();
						let sub_name =
							match split.next() {
								Some(s) if s.starts_with('.') =>
								&s[1..],
								_ => name,
							};
						(sub_name,
							array_id *
								(size_of::<ItemLocals>() as ElemOffset))
					}
					None => (name, 0),
				}
			};
		match sub_name {
			"uTransform" =>
			Some(Element{format:
								<[[f32; 4]; 4] as
									::shade::Formatted>::get_format(),
							offset:
								(((&tmp.transform as *const _ as usize) -
									base) as ElemOffset) +
									big_offset,}),
			"uPos" =>
			Some(Element{format:
								<[f32; 2] as
									::shade::Formatted>::get_format(),
							offset:
								(((&tmp.pos as *const _ as usize) - base)
									as ElemOffset) + big_offset,}),
			"uSize" =>
			Some(Element{format:
								<[f32; 2] as
									::shade::Formatted>::get_format(),
							offset:
								(((&tmp.size as *const _ as usize) -
									base) as ElemOffset) +
									big_offset,}),
			_ => None,
		}
	}
}
#[allow(missing_docs)]
#[rustc_copy_clone_marker]
pub struct Locals {
	pub gradient: f32,
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::clone::Clone for Locals {
	#[inline]
	fn clone(&self) -> Locals {
		{ let _: ::std::clone::AssertParamIsClone<f32>; *self }
	}
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::marker::Copy for Locals { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::fmt::Debug for Locals {
	fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
		-> ::std::fmt::Result {
		match *self {
			Locals { gradient: ref __self_0_0 } => {
				let mut builder = __arg_0.debug_struct("Locals");
				let _ = builder.field("gradient", &&(*__self_0_0));
				builder.finish()
			}
		}
	}
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(missing_docs)]
impl ::std::cmp::PartialEq for Locals {
	#[inline]
	fn eq(&self, __arg_0: &Locals) -> bool {
		match *__arg_0 {
			Locals { gradient: ref __self_1_0 } =>
			match *self {
				Locals { gradient: ref __self_0_0 } =>
				true && (*__self_0_0) == (*__self_1_0),
			},
		}
	}
	#[inline]
	fn ne(&self, __arg_0: &Locals) -> bool {
		match *__arg_0 {
			Locals { gradient: ref __self_1_0 } =>
			match *self {
				Locals { gradient: ref __self_0_0 } =>
				false || (*__self_0_0) != (*__self_1_0),
			},
		}
	}
}
unsafe impl ::traits::Pod for Locals { }
impl ::pso::buffer::Structure<::shade::ConstFormat> for Locals {
	fn query(name: &str)
		->
			::std::option::Option<::pso::buffer::Element<::shade::ConstFormat>> {
		use std::mem::{size_of, transmute};
		use ::pso::buffer::{Element, ElemOffset};
		let tmp: &Locals = unsafe { transmute(1usize) };
		let base = tmp as *const _ as usize;
		let (sub_name, big_offset) =
			{
				let mut split = name.split(|c| c == '[' || c == ']');
				let _ = split.next().unwrap();
				match split.next() {
					Some(s) => {
						let array_id: ElemOffset = s.parse().unwrap();
						let sub_name =
							match split.next() {
								Some(s) if s.starts_with('.') =>
								&s[1..],
								_ => name,
							};
						(sub_name,
							array_id *
								(size_of::<Locals>() as ElemOffset))
					}
					None => (name, 0),
				}
			};
		match sub_name {
			"uGradient" =>
			Some(Element{format:
								<f32 as
									::shade::Formatted>::get_format(),
							offset:
								(((&tmp.gradient as *const _ as usize) -
									base) as ElemOffset) +
									big_offset,}),
			_ => None,
		}
	}
}
#[allow(missing_docs)]
pub mod pipe {
	#[allow(unused_imports)]
	use super::*;
	use super::gfx;
	use ::pso::{DataLink, DataBind, Descriptor, InitError, RawDataSet,
				AccessInfo};
	pub struct Data<R: ::Resources> {
		pub vertex_buffer: <gfx::VertexBuffer<Vertex> as
							DataBind<R>>::Data,
		pub item_locals: <gfx::ConstantBuffer<ItemLocals> as
							DataBind<R>>::Data,
		pub locals: <gfx::ConstantBuffer<Locals> as
					DataBind<R>>::Data,
		pub out_target: <gfx::RenderTarget<ColourFormat> as
						DataBind<R>>::Data,
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl <R: ::std::clone::Clone + ::Resources> ::std::clone::Clone
		for Data<R> {
		#[inline]
		fn clone(&self) -> Data<R> {
			match *self {
				Data {
				vertex_buffer: ref __self_0_0,
				item_locals: ref __self_0_1,
				locals: ref __self_0_2,
				out_target: ref __self_0_3 } =>
				Data{vertex_buffer:
							::std::clone::Clone::clone(&(*__self_0_0)),
						item_locals:
							::std::clone::Clone::clone(&(*__self_0_1)),
						locals:
							::std::clone::Clone::clone(&(*__self_0_2)),
						out_target:
							::std::clone::Clone::clone(&(*__self_0_3)),},
			}
		}
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl <R: ::std::fmt::Debug + ::Resources> ::std::fmt::Debug for
		Data<R> {
		fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
			-> ::std::fmt::Result {
			match *self {
				Data {
				vertex_buffer: ref __self_0_0,
				item_locals: ref __self_0_1,
				locals: ref __self_0_2,
				out_target: ref __self_0_3 } => {
					let mut builder = __arg_0.debug_struct("Data");
					let _ =
						builder.field("vertex_buffer",
										&&(*__self_0_0));
					let _ =
						builder.field("item_locals", &&(*__self_0_1));
					let _ = builder.field("locals", &&(*__self_0_2));
					let _ =
						builder.field("out_target", &&(*__self_0_3));
					builder.finish()
				}
			}
		}
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl <R: ::std::cmp::PartialEq + ::Resources>
		::std::cmp::PartialEq for Data<R> {
		#[inline]
		fn eq(&self, __arg_0: &Data<R>) -> bool {
			match *__arg_0 {
				Data {
				vertex_buffer: ref __self_1_0,
				item_locals: ref __self_1_1,
				locals: ref __self_1_2,
				out_target: ref __self_1_3 } =>
				match *self {
					Data {
					vertex_buffer: ref __self_0_0,
					item_locals: ref __self_0_1,
					locals: ref __self_0_2,
					out_target: ref __self_0_3 } =>
					true && (*__self_0_0) == (*__self_1_0) &&
						(*__self_0_1) == (*__self_1_1) &&
						(*__self_0_2) == (*__self_1_2) &&
						(*__self_0_3) == (*__self_1_3),
				},
			}
		}
		#[inline]
		fn ne(&self, __arg_0: &Data<R>) -> bool {
			match *__arg_0 {
				Data {
				vertex_buffer: ref __self_1_0,
				item_locals: ref __self_1_1,
				locals: ref __self_1_2,
				out_target: ref __self_1_3 } =>
				match *self {
					Data {
					vertex_buffer: ref __self_0_0,
					item_locals: ref __self_0_1,
					locals: ref __self_0_2,
					out_target: ref __self_0_3 } =>
					false || (*__self_0_0) != (*__self_1_0) ||
						(*__self_0_1) != (*__self_1_1) ||
						(*__self_0_2) != (*__self_1_2) ||
						(*__self_0_3) != (*__self_1_3),
				},
			}
		}
	}
	pub struct Meta {
		vertex_buffer: gfx::VertexBuffer<Vertex>,
		item_locals: gfx::ConstantBuffer<ItemLocals>,
		locals: gfx::ConstantBuffer<Locals>,
		out_target: gfx::RenderTarget<ColourFormat>,
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl ::std::clone::Clone for Meta {
		#[inline]
		fn clone(&self) -> Meta {
			match *self {
				Meta {
				vertex_buffer: ref __self_0_0,
				item_locals: ref __self_0_1,
				locals: ref __self_0_2,
				out_target: ref __self_0_3 } =>
				Meta{vertex_buffer:
							::std::clone::Clone::clone(&(*__self_0_0)),
						item_locals:
							::std::clone::Clone::clone(&(*__self_0_1)),
						locals:
							::std::clone::Clone::clone(&(*__self_0_2)),
						out_target:
							::std::clone::Clone::clone(&(*__self_0_3)),},
			}
		}
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl ::std::fmt::Debug for Meta {
		fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
			-> ::std::fmt::Result {
			match *self {
				Meta {
				vertex_buffer: ref __self_0_0,
				item_locals: ref __self_0_1,
				locals: ref __self_0_2,
				out_target: ref __self_0_3 } => {
					let mut builder = __arg_0.debug_struct("Meta");
					let _ =
						builder.field("vertex_buffer",
										&&(*__self_0_0));
					let _ =
						builder.field("item_locals", &&(*__self_0_1));
					let _ = builder.field("locals", &&(*__self_0_2));
					let _ =
						builder.field("out_target", &&(*__self_0_3));
					builder.finish()
				}
			}
		}
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl ::std::cmp::PartialEq for Meta {
		#[inline]
		fn eq(&self, __arg_0: &Meta) -> bool {
			match *__arg_0 {
				Meta {
				vertex_buffer: ref __self_1_0,
				item_locals: ref __self_1_1,
				locals: ref __self_1_2,
				out_target: ref __self_1_3 } =>
				match *self {
					Meta {
					vertex_buffer: ref __self_0_0,
					item_locals: ref __self_0_1,
					locals: ref __self_0_2,
					out_target: ref __self_0_3 } =>
					true && (*__self_0_0) == (*__self_1_0) &&
						(*__self_0_1) == (*__self_1_1) &&
						(*__self_0_2) == (*__self_1_2) &&
						(*__self_0_3) == (*__self_1_3),
				},
			}
		}
		#[inline]
		fn ne(&self, __arg_0: &Meta) -> bool {
			match *__arg_0 {
				Meta {
				vertex_buffer: ref __self_1_0,
				item_locals: ref __self_1_1,
				locals: ref __self_1_2,
				out_target: ref __self_1_3 } =>
				match *self {
					Meta {
					vertex_buffer: ref __self_0_0,
					item_locals: ref __self_0_1,
					locals: ref __self_0_2,
					out_target: ref __self_0_3 } =>
					false || (*__self_0_0) != (*__self_1_0) ||
						(*__self_0_1) != (*__self_1_1) ||
						(*__self_0_2) != (*__self_1_2) ||
						(*__self_0_3) != (*__self_1_3),
				},
			}
		}
	}
	pub struct Init<'a> {
		pub vertex_buffer: <gfx::VertexBuffer<Vertex> as
							DataLink<'a>>::Init,
		pub item_locals: <gfx::ConstantBuffer<ItemLocals> as
							DataLink<'a>>::Init,
		pub locals: <gfx::ConstantBuffer<Locals> as
					DataLink<'a>>::Init,
		pub out_target: <gfx::RenderTarget<ColourFormat> as
						DataLink<'a>>::Init,
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl <'a> ::std::clone::Clone for Init<'a> {
		#[inline]
		fn clone(&self) -> Init<'a> {
			match *self {
				Init {
				vertex_buffer: ref __self_0_0,
				item_locals: ref __self_0_1,
				locals: ref __self_0_2,
				out_target: ref __self_0_3 } =>
				Init{vertex_buffer:
							::std::clone::Clone::clone(&(*__self_0_0)),
						item_locals:
							::std::clone::Clone::clone(&(*__self_0_1)),
						locals:
							::std::clone::Clone::clone(&(*__self_0_2)),
						out_target:
							::std::clone::Clone::clone(&(*__self_0_3)),},
			}
		}
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl <'a> ::std::fmt::Debug for Init<'a> {
		fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
			-> ::std::fmt::Result {
			match *self {
				Init {
				vertex_buffer: ref __self_0_0,
				item_locals: ref __self_0_1,
				locals: ref __self_0_2,
				out_target: ref __self_0_3 } => {
					let mut builder = __arg_0.debug_struct("Init");
					let _ =
						builder.field("vertex_buffer",
										&&(*__self_0_0));
					let _ =
						builder.field("item_locals", &&(*__self_0_1));
					let _ = builder.field("locals", &&(*__self_0_2));
					let _ =
						builder.field("out_target", &&(*__self_0_3));
					builder.finish()
				}
			}
		}
	}
	#[automatically_derived]
	#[allow(unused_qualifications)]
	impl <'a> ::std::cmp::PartialEq for Init<'a> {
		#[inline]
		fn eq(&self, __arg_0: &Init<'a>) -> bool {
			match *__arg_0 {
				Init {
				vertex_buffer: ref __self_1_0,
				item_locals: ref __self_1_1,
				locals: ref __self_1_2,
				out_target: ref __self_1_3 } =>
				match *self {
					Init {
					vertex_buffer: ref __self_0_0,
					item_locals: ref __self_0_1,
					locals: ref __self_0_2,
					out_target: ref __self_0_3 } =>
					true && (*__self_0_0) == (*__self_1_0) &&
						(*__self_0_1) == (*__self_1_1) &&
						(*__self_0_2) == (*__self_1_2) &&
						(*__self_0_3) == (*__self_1_3),
				},
			}
		}
		#[inline]
		fn ne(&self, __arg_0: &Init<'a>) -> bool {
			match *__arg_0 {
				Init {
				vertex_buffer: ref __self_1_0,
				item_locals: ref __self_1_1,
				locals: ref __self_1_2,
				out_target: ref __self_1_3 } =>
				match *self {
					Init {
					vertex_buffer: ref __self_0_0,
					item_locals: ref __self_0_1,
					locals: ref __self_0_2,
					out_target: ref __self_0_3 } =>
					false || (*__self_0_0) != (*__self_1_0) ||
						(*__self_0_1) != (*__self_1_1) ||
						(*__self_0_2) != (*__self_1_2) ||
						(*__self_0_3) != (*__self_1_3),
				},
			}
		}
	}
	impl <'a> ::pso::PipelineInit for Init<'a> {
		type
		Meta
		=
		Meta;
		fn link_to<'s>(&self, desc: &mut Descriptor,
						info: &'s ::ProgramInfo)
			-> ::std::result::Result<Self::Meta, InitError<&'s str>> {
			let mut meta =
				Meta{vertex_buffer:
							<gfx::VertexBuffer<Vertex> as
								DataLink<'a>>::new(),
						item_locals:
							<gfx::ConstantBuffer<ItemLocals> as
								DataLink<'a>>::new(),
						locals:
							<gfx::ConstantBuffer<Locals> as
								DataLink<'a>>::new(),
						out_target:
							<gfx::RenderTarget<ColourFormat> as
								DataLink<'a>>::new(),};
			let mut _num_vb = 0;
			if let Some(d) =
					meta.vertex_buffer.link_vertex_buffer(_num_vb,
															&self.vertex_buffer)
					{
				if !meta.vertex_buffer.is_active() {
					{
						::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
											{
												static _FILE_LINE:
														(&'static str,
														u32) =
													("src/ui/mainloop_gfx_sdl.rs",
													116u32);
												&_FILE_LINE
											})
					}
				};
				desc.vertex_buffers[_num_vb as usize] = Some(d);
				_num_vb += 1;
			}
			if let Some(d) =
					meta.item_locals.link_vertex_buffer(_num_vb,
														&self.item_locals)
					{
				if !meta.item_locals.is_active() {
					{
						::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
											{
												static _FILE_LINE:
														(&'static str,
														u32) =
													("src/ui/mainloop_gfx_sdl.rs",
													116u32);
												&_FILE_LINE
											})
					}
				};
				desc.vertex_buffers[_num_vb as usize] = Some(d);
				_num_vb += 1;
			}
			if let Some(d) = meta.locals.link_vertex_buffer(_num_vb, &self.locals) {
				assert!(meta.locals.is_active());
				desc.vertex_buffers[_num_vb as usize] = Some(d);
				_num_vb += 1;
			}
			if let Some(d) =
					meta.out_target.link_vertex_buffer(_num_vb,
														&self.out_target)
					{
				if !meta.out_target.is_active() {
					{
						::rt::begin_panic("assertion failed: meta.out_target.is_active()",
											{
												static _FILE_LINE:
														(&'static str,
														u32) =
													("src/ui/mainloop_gfx_sdl.rs",
													116u32);
												&_FILE_LINE
											})
					}
				};
				desc.vertex_buffers[_num_vb as usize] = Some(d);
				_num_vb += 1;
			}
			for at in &info.vertex_attributes {
				match meta.vertex_buffer.link_input(at,
													&self.vertex_buffer)
					{
					Some(Ok(d)) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.attributes[at.slot as usize] = Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::VertexImport(&at.name,
														Some(fm))),
					None => (),
				}
				match meta.item_locals.link_input(at,
													&self.item_locals) {
					Some(Ok(d)) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.attributes[at.slot as usize] = Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::VertexImport(&at.name,
														Some(fm))),
					None => (),
				}
				match meta.locals.link_input(at, &self.locals) {
					Some(Ok(d)) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.attributes[at.slot as usize] = Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::VertexImport(&at.name,
														Some(fm))),
					None => (),
				}
				match meta.out_target.link_input(at, &self.out_target)
					{
					Some(Ok(d)) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.attributes[at.slot as usize] = Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::VertexImport(&at.name,
														Some(fm))),
					None => (),
				}
				return Err(InitError::VertexImport(&at.name, None));
			}
			for cb in &info.constant_buffers {
				match meta.vertex_buffer.link_constant_buffer(cb,
																&self.vertex_buffer)
					{
					Some(Ok(d)) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.constant_buffers[cb.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::ConstantBuffer(&cb.name,
															Some(e))),
					None => (),
				}
				match meta.item_locals.link_constant_buffer(cb,
															&self.item_locals)
					{
					Some(Ok(d)) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.constant_buffers[cb.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::ConstantBuffer(&cb.name,
															Some(e))),
					None => (),
				}
				match meta.locals.link_constant_buffer(cb,
														&self.locals) {
					Some(Ok(d)) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.constant_buffers[cb.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::ConstantBuffer(&cb.name,
															Some(e))),
					None => (),
				}
				match meta.out_target.link_constant_buffer(cb,
															&self.out_target)
					{
					Some(Ok(d)) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.constant_buffers[cb.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::ConstantBuffer(&cb.name,
															Some(e))),
					None => (),
				}
				return Err(InitError::ConstantBuffer(&cb.name, None));
			}
			for gc in &info.globals {
				match meta.vertex_buffer.link_global_constant(gc,
																&self.vertex_buffer)
					{
					Some(Ok(())) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::GlobalConstant(&gc.name,
															Some(e))),
					None => (),
				}
				match meta.item_locals.link_global_constant(gc,
															&self.item_locals)
					{
					Some(Ok(())) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::GlobalConstant(&gc.name,
															Some(e))),
					None => (),
				}
				match meta.locals.link_global_constant(gc,
														&self.locals) {
					Some(Ok(())) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::GlobalConstant(&gc.name,
															Some(e))),
					None => (),
				}
				match meta.out_target.link_global_constant(gc,
															&self.out_target)
					{
					Some(Ok(())) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						continue ;
					}
					Some(Err(e)) =>
					return Err(InitError::GlobalConstant(&gc.name,
															Some(e))),
					None => (),
				}
				return Err(InitError::GlobalConstant(&gc.name, None));
			}
			for srv in &info.textures {
				match meta.vertex_buffer.link_resource_view(srv,
															&self.vertex_buffer)
					{
					Some(Ok(d)) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.resource_views[srv.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::ResourceView(&srv.name,
														Some(()))),
					None => (),
				}
				match meta.item_locals.link_resource_view(srv,
															&self.item_locals)
					{
					Some(Ok(d)) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.resource_views[srv.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::ResourceView(&srv.name,
														Some(()))),
					None => (),
				}
				match meta.locals.link_resource_view(srv,
														&self.locals) {
					Some(Ok(d)) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.resource_views[srv.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::ResourceView(&srv.name,
														Some(()))),
					None => (),
				}
				match meta.out_target.link_resource_view(srv,
															&self.out_target)
					{
					Some(Ok(d)) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.resource_views[srv.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::ResourceView(&srv.name,
														Some(()))),
					None => (),
				}
				return Err(InitError::ResourceView(&srv.name, None));
			}
			for uav in &info.unordereds {
				match meta.vertex_buffer.link_unordered_view(uav,
																&self.vertex_buffer)
					{
					Some(Ok(d)) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.unordered_views[uav.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::UnorderedView(&uav.name,
														Some(()))),
					None => (),
				}
				match meta.item_locals.link_unordered_view(uav,
															&self.item_locals)
					{
					Some(Ok(d)) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.unordered_views[uav.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::UnorderedView(&uav.name,
														Some(()))),
					None => (),
				}
				match meta.locals.link_unordered_view(uav,
														&self.locals) {
					Some(Ok(d)) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.unordered_views[uav.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::UnorderedView(&uav.name,
														Some(()))),
					None => (),
				}
				match meta.out_target.link_unordered_view(uav,
															&self.out_target)
					{
					Some(Ok(d)) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.unordered_views[uav.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(_)) =>
					return Err(InitError::UnorderedView(&uav.name,
														Some(()))),
					None => (),
				}
				return Err(InitError::UnorderedView(&uav.name, None));
			}
			for sm in &info.samplers {
				match meta.vertex_buffer.link_sampler(sm,
														&self.vertex_buffer)
					{
					Some(d) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.samplers[sm.slot as usize] = Some(d);
						continue ;
					}
					None => (),
				}
				match meta.item_locals.link_sampler(sm,
													&self.item_locals)
					{
					Some(d) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.samplers[sm.slot as usize] = Some(d);
						continue ;
					}
					None => (),
				}
				match meta.locals.link_sampler(sm, &self.locals) {
					Some(d) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.samplers[sm.slot as usize] = Some(d);
						continue ;
					}
					None => (),
				}
				match meta.out_target.link_sampler(sm,
													&self.out_target) {
					Some(d) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.samplers[sm.slot as usize] = Some(d);
						continue ;
					}
					None => (),
				}
				return Err(InitError::Sampler(&sm.name, None));
			}
			for out in &info.outputs {
				match meta.vertex_buffer.link_output(out,
														&self.vertex_buffer)
					{
					Some(Ok(d)) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&out.name,
														Some(fm))),
					None => (),
				}
				match meta.item_locals.link_output(out,
													&self.item_locals)
					{
					Some(Ok(d)) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&out.name,
														Some(fm))),
					None => (),
				}
				match meta.locals.link_output(out, &self.locals) {
					Some(Ok(d)) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&out.name,
														Some(fm))),
					None => (),
				}
				match meta.out_target.link_output(out,
													&self.out_target) {
					Some(Ok(d)) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						continue ;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&out.name,
														Some(fm))),
					None => (),
				}
				return Err(InitError::PixelExport(&out.name, None));
			}
			if !info.knows_outputs {
				use ::shade::core as s;
				let mut out =
					s::OutputVar{name: String::new(),
									slot: 0,
									base_type: s::BaseType::F32,
									container:
										s::ContainerType::Vector(4),};
				match meta.vertex_buffer.link_output(&out,
														&self.vertex_buffer)
					{
					Some(Ok(d)) => {
						if !meta.vertex_buffer.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						out.slot += 1;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&"!known",
														Some(fm))),
					None => (),
				}
				match meta.item_locals.link_output(&out,
													&self.item_locals)
					{
					Some(Ok(d)) => {
						if !meta.item_locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						out.slot += 1;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&"!known",
														Some(fm))),
					None => (),
				}
				match meta.locals.link_output(&out, &self.locals) {
					Some(Ok(d)) => {
						if !meta.locals.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.locals.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						out.slot += 1;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&"!known",
														Some(fm))),
					None => (),
				}
				match meta.out_target.link_output(&out,
													&self.out_target) {
					Some(Ok(d)) => {
						if !meta.out_target.is_active() {
							{
								::rt::begin_panic("assertion failed: meta.out_target.is_active()",
													{
														static _FILE_LINE:
																(&'static str,
																u32) =
															("src/ui/mainloop_gfx_sdl.rs",
															116u32);
														&_FILE_LINE
													})
							}
						};
						desc.color_targets[out.slot as usize] =
							Some(d);
						out.slot += 1;
					}
					Some(Err(fm)) =>
					return Err(InitError::PixelExport(&"!known",
														Some(fm))),
					None => (),
				}
			}
			for _ in 0..1 {
				if let Some(d) =
						meta.vertex_buffer.link_depth_stencil(&self.vertex_buffer)
						{
					if !meta.vertex_buffer.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.depth_stencil = Some(d);
				}
				if meta.vertex_buffer.link_scissor() {
					if !meta.vertex_buffer.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.vertex_buffer.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.scissor = true;
				}
				if let Some(d) =
						meta.item_locals.link_depth_stencil(&self.item_locals)
						{
					if !meta.item_locals.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.depth_stencil = Some(d);
				}
				if meta.item_locals.link_scissor() {
					if !meta.item_locals.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.item_locals.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.scissor = true;
				}
				if let Some(d) =
						meta.locals.link_depth_stencil(&self.locals) {
					if !meta.locals.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.locals.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.depth_stencil = Some(d);
				}
				if meta.locals.link_scissor() {
					if !meta.locals.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.locals.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.scissor = true;
				}
				if let Some(d) =
						meta.out_target.link_depth_stencil(&self.out_target)
						{
					if !meta.out_target.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.out_target.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.depth_stencil = Some(d);
				}
				if meta.out_target.link_scissor() {
					if !meta.out_target.is_active() {
						{
							::rt::begin_panic("assertion failed: meta.out_target.is_active()",
												{
													static _FILE_LINE:
															(&'static str,
															u32) =
														("src/ui/mainloop_gfx_sdl.rs",
														116u32);
													&_FILE_LINE
												})
						}
					};
					desc.scissor = true;
				}
			}
			Ok(meta)
		}
	}
	impl <R: ::Resources> ::pso::PipelineData<R> for Data<R> {
		type
		Meta
		=
		Meta;
		fn bake_to(&self, out: &mut RawDataSet<R>, meta: &Self::Meta,
					man: &mut ::handle::Manager<R>,
					access: &mut AccessInfo<R>) {
			meta.vertex_buffer.bind_to(out, &self.vertex_buffer, man,
										access);
			meta.item_locals.bind_to(out, &self.item_locals, man,
										access);
			meta.locals.bind_to(out, &self.locals, man, access);
			meta.out_target.bind_to(out, &self.out_target, man,
									access);
		}
	}
	pub fn new() -> Init<'static> {
		Init{vertex_buffer: (),
				item_locals: "ItemLocals",
				locals: "Locals",
				out_target: "Target0",}
	}
}
