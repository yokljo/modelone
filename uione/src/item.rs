use crate::geometry::Geometry;
use crate::resources::GraphicResourceManager;
use crate::vec2::*;

use modelone::impl_changeable_struct;
use modelone::model::{Change, Changeable, Watcher};
use modelone::object::{ApplyContext, ApplyHandle, JustSignal, JustSignalChange, Object};
use modelone::change_value::ValueChange;
use modelone::change_option::OptionChange;

use std::fmt;
use std::cell::RefCell;
use std::sync::mpsc;

use bitflags::bitflags;
//use std::sync::{Arc, RwLock};

/*pub struct ItemCache {
	damaged_items: Vec<usize>,
	
}*/

/*pub trait ItemCacheInterface: Send + Sync {
	fn mark_damaged(&mut self, id: usize);
	fn notify_children_added(&mut self, id: usize, indices: &[usize]);
	fn notify_children_removed(&mut self, id: usize, indices: &[usize]);
	fn update_apply_handle(&mut self, id: usize, apply_handle: ApplyHandle<ItemDataChange>);
}*/

pub enum ItemUpdateMessage {
	MarkDamaged{id: usize},
	UpdateApplyHandle{id: usize, apply_handle: ApplyHandle<ItemDataChange>},
	ChildrenAdded{id: usize, indices: Vec<usize>},
	ChildrenRemoved{id: usize, indices: Vec<usize>},
	AnimationStarted{id: usize},
	AnimationStopped{id: usize},
}

/// This must be implemented for all UI objects. It represents a rectangle on
/// the screen that can optionally have a mesh with a shader inside it.
pub trait Item: fmt::Debug {
	fn get_item(&self) -> &ItemData;
	//fn get_item_change(&self) -> Self;
	//fn get_item_mut(&mut self) -> &mut ItemData;
	//fn mark_damaged(&self) {
		// This would tell the scene graph to re-render this item.
		//self.get_item().internal.unwrap().cache.mark_damaged(self);
		//self.get_item().mark_damaged();
	//}
	fn update_geometry(&self, _old_geometry: Option<Box<Geometry>>, _resource_manager: &mut GraphicResourceManager) -> Option<Box<Geometry>> { None }
	fn apply_to_children(&self, apply: &mut FnMut(&[&Item])) { apply(&[]); }
}

/*
fn apply_to_children(apply: &mut FnMut(&Item, usize)) {
	apply(child1, 0);
	apply(child2, 1);
}
fn apply_to_child(apply: &mut FnMut(&Item), index: usize) {
	match index {
		0 => apply(child1, 0);
		1 => apply(child2, 1);
	}
}
fn child_count() -> usize {
	2
}
*/

#[macro_export]
macro_rules! impl_get_item{
	($name:ident) => {
		fn get_item(&self) -> &ItemData {
			&self.$name
		}
		/*fn get_item_mut(&mut self) -> &mut ItemData {
			&mut self.$name
		}*/
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemResetWatcher;

/// The most basic of changes, which just swaps the value with another
#[derive(Debug, Clone, PartialEq)]
pub struct ItemResetWatcherChange;

impl Change for ItemResetWatcherChange {
	type SignalType = ();
}

impl Changeable<ItemResetWatcherChange> for ItemResetWatcher {
	fn changeable_apply(&mut self, _change: ItemResetWatcherChange, _watcher: &mut Watcher<()>) {}
	
	fn reset_view_signals(&self) -> Vec<()> {
		vec![(),]
	}
}

pub struct ItemDataInternal {
	/// This would start off as None, but then be set to a unique identifier by
	/// the scene graph so it can be referred to in subsequent communications
	/// with the scene graph.
	pub id: usize,
	pub message_sender: mpsc::Sender<ItemUpdateMessage>,
}

impl fmt::Debug for ItemDataInternal {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "id: {}", self.id)
	}
}

impl PartialEq for ItemDataInternal {
	fn eq(&self, _: &ItemDataInternal) -> bool {
		true
	}
}

pub enum MouseButton {
	None,
	Left,
	Right,
	Middle,
}

impl MouseButton {
	pub fn to_mouse_buttons(&self) -> MouseButtons {
		match *self {
			MouseButton::None => MouseButtons::None,
			MouseButton::Left => MouseButtons::Left,
			MouseButton::Right => MouseButtons::Right,
			MouseButton::Middle => MouseButtons::Middle,
		}
	}
}

bitflags! {
	pub struct MouseButtons: u8 {
		#[allow(non_upper_case_globals)]
		const None   = 0b00000000;
		#[allow(non_upper_case_globals)]
		const Left   = 0b00000001;
		#[allow(non_upper_case_globals)]
		const Right  = 0b00000010;
		#[allow(non_upper_case_globals)]
		const Middle = 0b00000100;
	}
}

impl MouseButtons {
	pub fn set_button(&mut self, button: MouseButton, pressed: bool) {
		if pressed {
			*self |= button.to_mouse_buttons()
		} else {
			*self &= !button.to_mouse_buttons()
		}
	}
	
	pub fn get_button(&self, button: MouseButton) -> bool {
		(*self & button.to_mouse_buttons()) != MouseButtons::None
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct MouseData {
	pub contains_mouse: bool,
	pub mouse_pos: Option<Vec2f>,
	pub pressed: MouseButtons,
	
	//pub on_wheel: JustSignal,
}

impl MouseData {
	pub fn new() -> MouseData {
		MouseData {
			contains_mouse: false,
			mouse_pos: None,
			pressed: MouseButtons::None,
		}
	}
}

impl_changeable_struct!{MouseDataChange[MouseDataSignal] for MouseData:
	contains_mouse: ValueChange<bool>,
	mouse_pos: ValueChange<Option<Vec2f>>,
	pressed: ValueChange<MouseButtons>,
	
	// The value is the wheel delta
	//on_wheel: JustSignalChange<f64>,
}

bitflags! {
	pub struct ItemFlags: u8 {
		const NONE                   = 0b00000000;
		const VISIBLE                = 0b00000001;
		const FOCUS                  = 0b00000010;
		/// Set this to always update the item's geometry immediately before drawing.
		const ALWAYS_UPDATE_GEOMETRY = 0b00000100;
		const EVERYTHING             = 0b00000111;
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemFlagsChange {
	pub new_flags: ItemFlags,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemFlagsSignal {
	pub changed_flags: ItemFlags,
}

impl Change for ItemFlagsChange {
	type SignalType = ItemFlagsSignal;
}

impl Changeable<ItemFlagsChange> for ItemFlags {
	fn changeable_apply(&mut self, change: ItemFlagsChange, watcher: &mut Watcher<ItemFlagsSignal>) {
		let changed_flags = *self ^ change.new_flags;
		*self = change.new_flags;
		watcher.send_signal(ItemFlagsSignal { changed_flags });
	}
	
	fn reset_view_signals(&self) -> Vec<ItemFlagsSignal> {
		vec![ItemFlagsSignal { changed_flags: ItemFlags::EVERYTHING }]
	}
}

#[derive(Debug, PartialEq)]
pub struct ItemData {
	pub pos: Vec2f,
	pub size: Vec2f,
	//pub visible: bool,
	pub opacity: f32,
	//pub focus: bool,
	pub flags: ItemFlags,
	pub mouse_data: Option<MouseData>,
	pub animation_frame: JustSignal,
	
	pub item_reset_watcher: ItemResetWatcher,
	pub internal: RefCell<Option<ItemDataInternal>>,
}

impl_changeable_struct!{ItemDataChange[ItemDataSignal] for ItemData:
	/*on_reset => (&mut item_data) {
	
	},*/
	pos: ValueChange<Vec2f>,
	size: ValueChange<Vec2f> => on_changed (&mut item_data) {
		if item_data.size.x < 0. {
			item_data.size.x = 0.;
		}
		if item_data.size.y < 0. {
			item_data.size.y = 0.;
		}
		//println!("Size changed: {:?}", m.size);
		//println!("Size changed");
		item_data.mark_damaged();
	},
	//visible: ValueChange<bool>,
	//focus: ValueChange<bool>,
	flags: ItemFlagsChange,
	mouse_data: OptionChange<MouseData, MouseDataChange>,
	animation_frame: JustSignalChange<f64>,
	
	//item_reset_watcher: ItemResetWatcherChange,
	item_reset_watcher: ItemResetWatcherChange => on_changed (&mut _item_data) {
		//println!("Item data reset");
		//item_data.
	},
}

impl ItemData {
	pub fn new() -> ItemData {
		ItemData {
			pos: Vec2f::default(),
			size: Vec2f::default(),
			//visible: true,
			opacity: 1.,
			//focus: false,
			flags: ItemFlags::VISIBLE,
			mouse_data: None,
			animation_frame: JustSignal,
			item_reset_watcher: ItemResetWatcher,
			internal: RefCell::new(None),
		}
	}
	
	pub fn new_mouse_area() -> ItemData {
		ItemData {
			pos: Vec2f::default(),
			size: Vec2f::default(),
			//visible: true,
			opacity: 1.,
			//focus: false,
			flags: ItemFlags::VISIBLE,
			mouse_data: Some(MouseData::new()),
			animation_frame: JustSignal,
			item_reset_watcher: ItemResetWatcher,
			internal: RefCell::new(None),
		}
	}
	
	pub fn mark_damaged(&self) {
		if let Some(ref internal) = *self.internal.borrow() {
			internal.message_sender.send(ItemUpdateMessage::MarkDamaged{id: internal.id}).ok();
		}
	}
	
	pub fn notify_children_added(&self, indices: &[usize]) {
		if let Some(ref internal) = *self.internal.borrow() {
			internal.message_sender.send(ItemUpdateMessage::ChildrenAdded{id: internal.id, indices: indices.into()}).ok();
		}
	}
	
	pub fn notify_children_removed(&self, indices: &[usize]) {
		if let Some(ref internal) = *self.internal.borrow() {
			internal.message_sender.send(ItemUpdateMessage::ChildrenRemoved{id: internal.id, indices: indices.into()}).ok();
		}
	}
	
	pub fn set_animating(&self, animating: bool) {
		if let Some(ref internal) = *self.internal.borrow() {
			if animating {
				internal.message_sender.send(ItemUpdateMessage::AnimationStarted{id: internal.id}).ok();
			} else {
				internal.message_sender.send(ItemUpdateMessage::AnimationStopped{id: internal.id}).ok();
			}
		}
	}
}

impl Object<ItemDataChange> for ItemData {
	fn update(&self, cxt: &mut ApplyContext<ItemDataChange>, signal: &ItemDataSignal) {
		if let ItemDataSignal::item_reset_watcher(_) = *signal {
			if self.internal.borrow().is_some() {
				let apply_handle = cxt.apply_handle();
				if let Some(ref internal) = *self.internal.borrow() {
					internal.message_sender.send(ItemUpdateMessage::UpdateApplyHandle{id: internal.id, apply_handle}).ok();
				}
			} else {
				println!("No item internal");
			}
			//println!("Item data reset");
		}
	}
}

pub struct SubItem<'f> {
	item: &'f Item,
	children: &'f [&'f Item],
}

impl<'f> SubItem<'f> {
	pub fn new(item: &'f Item, children: &'f [&'f Item]) -> SubItem<'f> {
		SubItem {item, children}
	}
}

impl<'f> Item for SubItem<'f> {
	fn get_item(&self) -> &ItemData {
		self.item.get_item()
	}
	
	fn update_geometry(&self, old_geometry: Option<Box<Geometry>>, resource_manager: &mut GraphicResourceManager) -> Option<Box<Geometry>> {
		self.item.update_geometry(old_geometry, resource_manager)
	}
	
	fn apply_to_children(&self, apply: &mut FnMut(&[&Item])) {
		apply(self.children);
	}
}

impl<'f> fmt::Debug for SubItem<'f> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SubItem")
	}
}
