pub mod cast_slice;
pub mod colour;
pub mod geometry;
pub mod image_data;
#[macro_use] pub mod resources;
#[macro_use] pub mod shadervalue;
#[macro_use] pub mod shader;
#[macro_use] pub mod item;
pub mod item_graphics_pipeline;
pub mod mainloop_gl_glutin;
pub mod anchors;
pub mod vec2;
pub mod rect;
pub mod basicitems;
pub mod gl_wrapper;

pub mod texture_resource;
pub mod font_resource;

pub use crate::colour::*;
pub use crate::image_data::{ImageData, ImageFormat};
pub use crate::item::*;
//pub use scenegraph::*;
pub use crate::shader::*;
pub use crate::anchors::*;
pub use crate::basicitems::rectangle::{Rectangle, RectangleChange, RectangleSignal};
pub use crate::basicitems::text::{Text, TextChange, TextSignal, TextFormat};
pub use crate::basicitems::image::{Image, ImageChange, ImageSignal};

use crate::vec2::*;
use crate::rect::*;

use std::cell::Cell;
use std::sync::Arc;
use std::iter::Iterator;
use std::any::Any;

use modelone::object::{ApplyContext, JustSignal, JustSignalChange, Object};
use modelone::change_option::OptionSignal;
use modelone::change_value::ValueChange;
use modelone::change_string::StringChange;
use modelone::history::{History, HistoryChange};
use modelone::{impl_changeable_struct, impl_revertable_struct, dispatch_struct_update, sub_apply};

/// This simplifies implementing the apply_to_children method of the Item trait.
#[macro_export] macro_rules! impl_children {
	(
		@impl $self:ident $name:ident
	) => {
		&$self.$name
	};
	
	(
		@impl $self:ident $name:ident {
			$(
				$subname:ident $({
					$($token:tt)*
				})*,
			)+
		}
	) => {
		&SubItem::new(&$self.$name, &[
			$(
				impl_children!(@impl $self $subname $({$($token)*})*),
			)*
		])
	};

	(
		$(
			$name:ident $({
				$($token:tt)*
			})*,
		)+
	) => {
		fn apply_to_children(&self, apply: &mut FnMut(&[&Item])) {
			apply(&[
				$(
					impl_children!(@impl self $name $({$($token)*})*),
				)*
			]);
		}
	};
}

#[derive(Debug, Clone)]
pub enum UiEvent {
	Text(String),
	Left,
	Right,
	Esc,
	CtrlZ,
	CtrlShiftZ,
}

#[derive(Debug, PartialEq)]
pub struct WindowData {
	borderless: bool,
	id: Option<Cell<usize>>,
}

impl_changeable_struct!{WindowDataChange[WindowDataSignal] for WindowData:
	borderless: ValueChange<bool>,
}

pub trait Window : Item {
	fn get_window(&self) -> &WindowData;
	fn get_window_mut(&mut self) -> &mut WindowData;
}

/*macro_rules! get_window_impl{
	($name:ident) => {
		fn get_window(&self) -> &WindowData {
			&self.$name
		}
		fn get_window_mut(&mut self) -> &mut WindowData {
			&mut self.$name
		}
	}
}*/

#[derive(Debug, PartialEq)]
pub struct Label {
	item_data: ItemData,
	text: String,
}

impl_changeable_struct!{LabelChange[LabelSignal] for Label:
	item_data: ItemDataChange,
	text: ValueChange<String>,
}

impl Item for Label {
	impl_get_item!(item_data);
}

struct Theme {
	bg_colour: Colour,
	fg_colour: Colour,
}

// There needs to be a good/simple way to make a trait which inherits the Object and Item traits
// that can be implemented on existing Item types, exposing a standard Box-able interface.
// For example, Button could be a trait, and when you want a button, Box<Button> can be used.
// Then libraries can provide their own arbitrarily complex implementations of button, and as long
// as they expose the correct interface, they can be used in place of the button in any program that
// uses Box<Button> for its buttons.
// Then, the actual button implementation that is used can be hidden from the user, and instead
// Box<Button> values can be retrieved from some theme system. To take it further, the actual button
// implementation could be swapped out while the program is running.

/*enum StandardButtonChange {
	item_data(ItemDataChange),
	text(Arc<String>),
	button_clicked(),
	internal(Box<Any>),
}

trait StandardButton: Item + Object<StandardButtonChange> {
	fn text(&self) -> &str;
}*/

#[derive(Debug, PartialEq)]
pub struct Button {
	pub item_data: ItemData,
	pub text: Arc<String>,
	pub back_rect: Rectangle,
	pub label: Text,
	pub clicked: JustSignal,
	// An ApplyHandle could be created for this signal, then passed to some kind of theme system
	// that is accessible from all items. Perhaps the Item trait needs another method that allows
	// propagating this type of resource (another example is translating strings) through a tree,
	// while allowing specific sub-trees to have different versions of the resource, so themes can
	// be applied per-tree.
	// When the theme changes, all it has to do is invoke all the registered ApplyHandles with the
	// new Arc<Theme>.
	//pub theme_changed: JustSignal,
	//pub theme: Arc<Theme>,
}

impl Button {
	pub fn new(text: Arc<String>) -> Button {
		Button {
			item_data: ItemData::new_mouse_area(),
			text: text.clone(),
			back_rect: Rectangle::new_colour(Colour::rgb(0.5, 0.5, 0.5)),
			label: Text {
				text: text.clone(),
				.. Text::new(Arc::new(TextFormat::new_size_family_colour(12., Arc::new("Arial".into()), std_colour::RED)))
			},
			clicked: JustSignal,
		}
	}
}

impl_changeable_struct!{ButtonChange[ButtonSignal] for Button:
	item_data: ItemDataChange,
	text: ValueChange<Arc<String>>,
	back_rect: RectangleChange,
	label: TextChange,
	clicked: JustSignalChange<()>,
	//theme_changed: JustSignalChange<Arc<Theme>>,
}

impl Item for Button {
	impl_get_item!(item_data);
	
	impl_children!{
		back_rect {
			label,
		},
	}
}

impl Object<ButtonChange> for Button {
	fn update(&self, cxt: &mut ApplyContext<ButtonChange>, signal: &ButtonSignal) {
		Anchors::new_fill(AnchorRelation::Parent, self)
			.apply(&self.back_rect.item_data, sub_apply!(cxt, ButtonChange::back_rect.RectangleChange::item_data));
			
		Anchors::new_fill_margin(AnchorRelation::Parent, &self.back_rect, 5.)
			.apply(&self.label.item_data, sub_apply!(cxt, ButtonChange::label.TextChange::item_data));
		
		if let ButtonSignal::item_data(ItemDataSignal::mouse_data(OptionSignal::Change(MouseDataSignal::contains_mouse(_)))) = *signal {
			if self.item_data.mouse_data.as_ref().unwrap().contains_mouse {
				cxt.apply(ButtonChange::back_rect(RectangleChange::border_colour(ValueChange(std_colour::LIME))));
			} else {
				cxt.apply(ButtonChange::back_rect(RectangleChange::border_colour(ValueChange(std_colour::RED))));
			}
		}
		
		if let ButtonSignal::item_data(ItemDataSignal::mouse_data(OptionSignal::Change(MouseDataSignal::pressed(_)))) = *signal {
			if self.item_data.mouse_data.as_ref().unwrap().pressed != MouseButtons::None {
				cxt.apply(ButtonChange::back_rect(RectangleChange::colour(ValueChange(Colour::rgb(1., 1., 1.)))));
			} else {
				cxt.apply(ButtonChange::back_rect(RectangleChange::colour(ValueChange(Colour::rgb(0.5, 0.5, 0.5)))));
				cxt.apply(ButtonChange::clicked(JustSignalChange(())));
			}
		}
		
		dispatch_struct_update!{ButtonChange[ButtonSignal] for self, cxt, signal:
			item_data: ItemData,
			back_rect: Rectangle,
			label: Text,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct TestWidget1Content {
	pub text: String,
	pub cursor_pos: usize,
	pub selection_pos: Option<usize>,
}

impl_revertable_struct!{TestWidget1ContentChange[TestWidget1ContentSignal] for TestWidget1Content:
	text: StringChange,
	cursor_pos: ValueChange<usize>,
	selection_pos: ValueChange<Option<usize>>,
}

/// A text field based around a String model that is changed with StringChange.
#[derive(Debug, PartialEq)]
pub struct TestWidget1 {
	pub item_data: ItemData,
	pub back_rect: Rectangle,
	pub inside_rect: Rectangle,
	pub content: History<TestWidget1Content, TestWidget1ContentChange>,
	pub text_item: Text,
	pub image: Image,
}

impl_changeable_struct!{TestWidget1Change[TestWidget1Signal] for TestWidget1:
	item_data: ItemDataChange,
	back_rect: RectangleChange,
	inside_rect: RectangleChange,
	content: HistoryChange<TestWidget1Content, TestWidget1ContentChange>,
	text_item: TextChange,
	image: ImageChange,
}

impl Item for TestWidget1 {
	impl_get_item!(item_data);
	
	impl_children!{
		back_rect {
			inside_rect,
			//text_item,
		},
	}
	
	/*fn item_event(&self, cxt: &mut ApplyContext<TestWidget1, TestWidget1Change>, signal: &TestWidget1Signal) {
		if let TestWidget1Signal::item_data(ItemDataSignal::contains_mouse(_)) = signal {
			if self.item_data.contains_mouse {
				println!("Mouse in");
			} else {
				println!("Mouse out");
			}
		}
	}*/
	
	/*fn apply_to_children(&self, apply: &mut FnMut(&[&Item])) {
		apply(&[
			&SubItem::new(&self.back_rect, &[
				&self.inside_rect
			]),
		]);
	}*/
}

/*#[derive(Debug)]
enum TestWidget1Signal {
	//Change(StringChange),
	Undo,
	Redo,
}*/

impl TestWidget1 {
	pub fn new() -> TestWidget1 {
		TestWidget1 {
			item_data: ItemData::new_mouse_area(),
			back_rect: Rectangle {
				item_data: ItemData::new(),
				colour: Colour::rgb(1., 0.5, 0.2),
				border_width: 2f64,
				border_colour: std_colour::LIME,
			},
			inside_rect: Rectangle {
				item_data: ItemData::new_mouse_area(),
				colour: std_colour::LIME,
				border_width: 2f64,
				border_colour: std_colour::RED,
			},
			content: History::new(TestWidget1Content {
				text: "".into(),
				cursor_pos: 0,
				selection_pos: None,
			}),
			text_item: Text {
				text: Arc::new("Hello World".into()),
				.. Text::new(Arc::new(TextFormat::new_size_family_colour(12., Arc::new("Arial".into()), std_colour::RED)))
			},
			image: Image {
				item_data: ItemData::new(),
				source: Arc::new(ImageData::new_rgb888(vec![(0,0,0), (255,255,255), (255,255,255), (0,0,0)], Vec2::new(2, 2)).unwrap()),
				//source: Arc::new(ImageData::new_rgba8888(vec![(0,0,0,255), (100,255,255,255), (255,255,255,255), (0,0,0,255)], Vec2::new(2, 2)).unwrap()),
			},
		}
	}
	
	/*pub fn event(cxt: &mut ApplyContext<TestWidget1, TestWidget1Change>, event: UiEvent) -> Vec<TestWidget1Signal> {
		if let UiEvent::Text(text) = event {
			let cursor_pos = self.content.model.cursor_pos;
			//cxt.apply(Content(Push(Text({index: cursor_pos, len: 0, new: text.clone()}))));
			cxt.apply(TestWidget1Change::content(HistoryChange::Push(TestWidget1ContentChange::text(StringChange{index: cursor_pos, len: 0, new: text.clone()}))));
			cxt.apply(TestWidget1Change::content(HistoryChange::Push(TestWidget1ContentChange::cursor_pos(ValueChange(cursor_pos + text.len())))));
		}
		vec![]
	}*/
	
	pub fn replace_text(&self, cxt: &mut ApplyContext<TestWidget1Change>, new_text: String) {
		let text_len = self.content.model.text.len();
		cxt.apply(TestWidget1Change::content(HistoryChange::Push(TestWidget1ContentChange::text(StringChange{index: 0, len: text_len, new: new_text}))));
	}
}

impl Object<TestWidget1Change> for TestWidget1 {
	// Repeatedly apply this function until it no longer emits signals, or a
	// loop is detected?
	fn update(&self, cxt: &mut ApplyContext<TestWidget1Change>, signal: &TestWidget1Signal) {
		//println!("{:?}", signal);
		//Anchors::new_fill(AnchorRelation::Parent, self).apply(&mut SubApplyContext::new(cxt, &|model| &model.back_rect.item_data, &mut |change| TestWidget1Change::back_rect(RectangleChange::item_data(change))));
		//Anchors::new_fill_margin(AnchorRelation::Parent, &self.back_rect, 30.).apply(&mut SubApplyContext::new(cxt, &|model| &model.inside_rect.item_data, &mut |change| TestWidget1Change::inside_rect(RectangleChange::item_data(change))));
		
		Anchors::new_fill(AnchorRelation::Parent, self)
			.apply(&self.back_rect.item_data, sub_apply!(cxt, TestWidget1Change::back_rect.RectangleChange::item_data));
		
		Anchors::new_fill_margin(AnchorRelation::Parent, &self.back_rect, 30.)
			.move_left_border(30.)
			.apply(&self.inside_rect.item_data, sub_apply!(cxt, TestWidget1Change::inside_rect.RectangleChange::item_data));
			
		Anchors::new_fill_margin(AnchorRelation::Parent, &self.back_rect, 170.)
			.apply(&self.image.item_data, sub_apply!(cxt, TestWidget1Change::image.ImageChange::item_data));
			
		Anchors::new_fill_margin(AnchorRelation::Parent, &self.back_rect, 170.)
			.move_bottom_border(200.)
			.apply(&self.text_item.item_data, sub_apply!(cxt, TestWidget1Change::text_item.TextChange::item_data));
		
		if let TestWidget1Signal::item_data(ItemDataSignal::animation_frame(step)) = *signal {
			if self.inside_rect.item_data.mouse_data.as_ref().unwrap().pressed != MouseButtons::None {
				let new_width = self.inside_rect.border_width + (step * 100.);
				cxt.apply(TestWidget1Change::inside_rect(RectangleChange::border_width(ValueChange(new_width))));
			}
		}
		
		if let TestWidget1Signal::inside_rect(RectangleSignal::item_data(ItemDataSignal::mouse_data(OptionSignal::Change(MouseDataSignal::contains_mouse(_))))) = *signal {
			if self.inside_rect.item_data.mouse_data.as_ref().unwrap().contains_mouse {
				cxt.apply(TestWidget1Change::inside_rect(RectangleChange::border_colour(ValueChange(std_colour::CYAN))));
			} else {
				cxt.apply(TestWidget1Change::inside_rect(RectangleChange::border_colour(ValueChange(std_colour::RED))));
			}
		}
		
		if let TestWidget1Signal::inside_rect(RectangleSignal::item_data(ItemDataSignal::mouse_data(OptionSignal::Change(MouseDataSignal::pressed(_))))) = *signal {
			if self.inside_rect.item_data.mouse_data.as_ref().unwrap().pressed != MouseButtons::None {
				cxt.apply(TestWidget1Change::inside_rect(RectangleChange::colour(ValueChange(std_colour::BLUE))));
				self.get_item().set_animating(true);
			} else {
				cxt.apply(TestWidget1Change::inside_rect(RectangleChange::colour(ValueChange(std_colour::LIME))));
				self.get_item().set_animating(false);
				cxt.apply(TestWidget1Change::inside_rect(RectangleChange::border_width(ValueChange(2.))));
			}
		}
		
		dispatch_struct_update!{TestWidget1Change[TestWidget1Signal] for self, cxt, signal:
			item_data: ItemData,
			back_rect: Rectangle,
			inside_rect: Rectangle,
			//content: History<TestWidget1Content, TestWidget1ContentChange>,
		}
	
		/*if let TestWidget1Signal::Content(..) = *signal {
			let border_width = self.content.model.text.len() as f64;
			cxt.apply(TestWidget1Change::BackRect(RectangleChange::BorderWidth(ValueChange(border_width))));
		}*/
		/*if let TestWidget1Signal::BackRect(RectangleSignal::ItemData(item_data_signal)) = signal {
			match item_data_signal {
				ItemDataSignal::Pos{..} | ItemDataSignal::Size{..} =>
					Anchors::new_fill_margin(&self, 2.).apply(&self.back_rect);
				_ => {}
			}
		}*/
		
		//self.text.changeable_apply(change.clone());
		//self.text = model.clone();
		self.get_item().mark_damaged();
	}
}

trait Positionable {
	fn rect(&self) -> Rectf;
	fn set_rect(&mut self, rect: Rectf);
}

trait Layout {
	/// Perform the layout, then return the bounds of the layed out items.
	fn layout(&self, iter: &mut Iterator<Item=&mut Positionable>, within: Rectf) -> Rectf;
}

#[derive(Clone, Debug)]
struct ColumnLayout {
	spacing: f64,
}

impl Layout for ColumnLayout {
	fn layout(&self, iter: &mut Iterator<Item=&mut Positionable>, within: Rectf) -> Rectf {
		let mut current = within.top_left();
		for positionable in iter {
			let rect = positionable.rect();
			positionable.set_rect(Rectf::new(current, rect.size));
			current.y += rect.height() + self.spacing;
		}
		Rectf::new(within.top_left(), Vec2f::new(within.width(), current.y))
	}
}

/*#[derive(Clone, Debug)]
struct Repeater<ItemType: Item, ModelTypeChange: Change, ModelType: Changeable<ModelTypeChange>, LayoutType: Layout + fmt::Debug> {
	items: Vec<ItemType>,
	layout: LayoutType,
}

#[derive(Clone, Debug)]
enum RepeaterChange {
	DoNothing,
}

#[derive(Clone, Debug, PartialEq)]
enum RepeaterSignal {
	DoSomething,
}

impl Change for RepeaterChange {
	type SignalType = RepeaterSignal;
	
	fn do_nothing() -> Self { RepeaterChange::DoNothing }
}*/

/*impl<ItemType: Item> Object<Repeater<ItemType>, RepeaterChange> for Repeater<ItemType> {
	fn update(cxt: &mut ApplyContext<Repeater<ItemType>, RepeaterChange>, signal: &RepeaterSignal) {
		
	}
}*/
