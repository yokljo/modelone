use crate::image_data::ImageData;
use crate::rect::Rect;

use std;
use std::fmt::Debug;
use std::sync::atomic::Ordering;
use std::any::Any;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::cell::{RefCell, Ref, RefMut};
use std::ops::{Deref, DerefMut};
use lazy_static::lazy_static;

static NEXT_RESOURCE_INDEX: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
	static ref RESOURCE_NAMES: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct GraphicResourceHandle(usize);

impl GraphicResourceHandle {
	pub fn get_id(self) -> usize {
		self.0
	}
}

/// Call this ONCE per type of resource and store the resource handle for that type for the rest of
/// time.
pub fn get_unique_resource_handle(name: String) -> GraphicResourceHandle {
	let mut names = RESOURCE_NAMES.lock().unwrap();
	names.push(name);
	GraphicResourceHandle(NEXT_RESOURCE_INDEX.fetch_add(1, Ordering::SeqCst))
}

pub fn get_resource_name(handle: GraphicResourceHandle) -> String {
	let names = RESOURCE_NAMES.lock().unwrap();
	names[handle.get_id()].clone()
}

/// A custom resource system that is shared between all items in a scene. This can be used to store
/// for example an image or font cache that all items can use when rendering, so the same thing
/// doesn't have to be loaded multiple times. The resources are shared per display context.
pub trait GraphicResource: Debug {
	fn as_any(&mut self) -> &mut Any;
	fn get_handle(&self) -> GraphicResourceHandle;
}

#[derive(Debug, Clone)]
pub enum GraphicResourceError {
	NoSuchResource,
	ResourceAlreadyInUse,
	ResourceHasWrongType,
}

pub struct GraphicResourceRef<'m, T: 'static> {
	graphic_resource_refmut: RefMut<'m, Box<GraphicResource>>,
	// As long as graphic_resource_refmut is alive, this pointer to the typed resource will be
	// alive, so it is safe to dereference it during that time.
	typed_resource: *mut T,
}

impl<'m, T: 'static> GraphicResourceRef<'m, T> {
	pub fn from_graphic_resource(graphic_resource_refmut: RefMut<'m, Box<GraphicResource>>) -> Result<GraphicResourceRef<'m, T>, GraphicResourceError> {
		let mut res = GraphicResourceRef {
			graphic_resource_refmut,
			typed_resource: std::ptr::null_mut(),
		};
		
		{
			let typed_resource_result = res.graphic_resource_refmut.deref_mut().as_any().downcast_mut::<T>();
			
			match typed_resource_result {
				Some(typed_resource) => {
					res.typed_resource = typed_resource;
				}
				None => {
					return Err(GraphicResourceError::ResourceHasWrongType);
				}
			}
		}
		
		Ok(res)
	}
}

impl<'m, T> Deref for GraphicResourceRef<'m, T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe {
			&*self.typed_resource
		}
	}
}

impl<'m, T> DerefMut for GraphicResourceRef<'m, T> {
	fn deref_mut(&mut self) -> &mut T {
		unsafe {
			&mut *self.typed_resource
		}
	}
}

#[macro_export] macro_rules! uione_graphic_resource {
	(
		$resource_type:ty, $getter_name:ident, $handle_name:ident
	) => {
		lazy_static::lazy_static! {
			pub static ref $handle_name: $crate::resources::GraphicResourceHandle = $crate::resources::get_unique_resource_handle(stringify!($resource_type).into());
		}

		impl $crate::resources::GraphicResource for $resource_type {
			fn as_any(&mut self) -> &mut ::std::any::Any {
				self
			}

			fn get_handle(&self) -> $crate::resources::GraphicResourceHandle {
				*$handle_name
			}
		}

		pub fn $getter_name<'m>(resource_manager: &'m $crate::resources::GraphicResourceManager) -> Result<$crate::resources::GraphicResourceRef<'m, $resource_type>, $crate::resources::GraphicResourceError> {
			let res = resource_manager.get_resource(*$handle_name)?;
			$crate::resources::GraphicResourceRef::from_graphic_resource(res)
			//res.as_any().downcast_mut::<$resource_type>().ok_or($crate::resources::GraphicResourceError::ResourceHasWrongType)
		}
	}
}

pub struct GraphicResourceManager {
	resources: Vec<Option<RefCell<Box<GraphicResource>>>>,
}

impl GraphicResourceManager {
	pub fn new() -> GraphicResourceManager {
		GraphicResourceManager {
			resources: vec![],
		}
	}
	
	pub fn register_resource(&mut self, resource: Box<GraphicResource>) -> Result<(), Box<GraphicResource>> {
		let id = resource.get_handle().get_id();
		while self.resources.len() <= id {
			self.resources.push(None);
		}
		
		if self.resources[id].is_some() {
			// The resource is already registered.
			return Err(resource);
		}
		
		println!("Register resource {:?} ({})", get_resource_name(resource.get_handle()), resource.get_handle().get_id());
		
		self.resources[id] = Some(RefCell::new(resource));
		
		Ok(())
	}
	
	/*fn make_texture(&self, image_data: &image_data::ImageData) -> Arc<TextureResource> {
		Arc::new(GlTextureResource::new(image_data).unwrap())
	}*/
	
	pub fn get_resource(&self, handle: GraphicResourceHandle) -> Result<RefMut<Box<GraphicResource>>, GraphicResourceError> {
		let id = handle.get_id();
		if let Some(Some(ref resource_cell)) = self.resources.get(id) {
			if let Ok(resource) = resource_cell.try_borrow_mut() {
				Ok(resource)
			} else {
				Err(GraphicResourceError::ResourceAlreadyInUse)
			}
		} else {
			Err(GraphicResourceError::NoSuchResource)
		}
	}
	
	/*pub fn get_resources<'l>(&mut self, requests: &mut [(GraphicResourceHandle, Result<RefMut<Box<GraphicResource>>, GraphicResourceError>)]) {
		for (handle, result) in &mut requests {
			if let Some(Some(ref resource_cell)) = self.resources.get(id) {
				if let Ok(resource) = resource_cell.try_borrow_mut() {
					Ok(resource)
				} else {
					Err(GraphicResourceError::ResourceAlreadyInUse)
				}
			} else {
				Err(GraphicResourceError::NoSuchResource)
			}
		}
	}*/
}

/*pub struct GraphicResourceQueryResult {
	manager: &mut GraphicResourceManager,
	results: &mut [(GraphicResourceHandle, Result<RefMut<Box<GraphicResource>>, GraphicResourceError>)]
}*/

/*pub trait ResourceManager {
	fn make_texture(&self, image_data: &ImageData) -> Arc<TextureResource>;
	fn get_custom_resource(&mut self, handle: GraphicResourceHandle) -> Result<&mut GraphicResource, GraphicResourceError>;
	//fn get_custom_resources(&mut self, resources: &mut [(GraphicResourceHandle, Option<&mut Any>)]);
}*/
