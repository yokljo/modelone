use crate::item::*;
use crate::rect::*;
use crate::vec2::*;

use modelone::object::ApplyContext;
use modelone::change_value::ValueChange;

/// Describes the relationship of one item to another.
pub enum AnchorRelation {
	Parent,
	Sibling,
}

/// These are choices of combinations to anchor to.
pub enum AnchorFrom {
	None,
	Start(f64),
	End(f64),
	StartEnd(f64, f64),
	Middle(f64),
}

// Each item is either None or Some((item, anchor position, margin)).
pub struct Anchors {
	horizontal: AnchorFrom,
	vertical: AnchorFrom,
}

impl Anchors {
	/// Create a new anchors instance that does nothing.
	pub fn new() -> Anchors {
		Anchors {
			horizontal: AnchorFrom::None,
			vertical: AnchorFrom::None,
		}
	}
	
	fn item_rect(relation: AnchorRelation, item: &Item) -> Rectf {
		match relation {
			AnchorRelation::Parent => Rect::new(Vec2::new(0., 0.), item.get_item().size),
			AnchorRelation::Sibling => Rect::new(item.get_item().pos, item.get_item().size),
		}
	}
	
	/// Create a new anchors instance that can be used to make another item fill
	/// the given item, with the given margin.
	pub fn new_fill_margin(relation: AnchorRelation, item: &Item, margin: f64) -> Anchors {
		let item_rect = Anchors::item_rect(relation, item);
		Anchors {
			horizontal: AnchorFrom::StartEnd(item_rect.left() + margin, item_rect.right() - margin),
			vertical: AnchorFrom::StartEnd(item_rect.top() + margin, item_rect.bottom() - margin),
		}
	}
	
	/// Create a new anchors instance that can be used to make another item fill
	/// the given item.
	pub fn new_fill(relation: AnchorRelation, item: &Item) -> Anchors {
		Anchors::new_fill_margin(relation, item, 0.)
	}
	
	/// Offset the left border by the given amount.
	pub fn move_left_border(&mut self, by: f64) -> &mut Self {
		match self.horizontal {
			AnchorFrom::Start(ref mut x) => *x += by,
			AnchorFrom::StartEnd(ref mut x, _) => *x += by,
			_ => {}
		}
		self
	}
	
	/// Offset the right border by the given amount.
	pub fn move_right_border(&mut self, by: f64) -> &mut Self {
		match self.horizontal {
			AnchorFrom::End(ref mut x) => *x -= by,
			AnchorFrom::StartEnd(_, ref mut x) => *x -= by,
			_ => {}
		}
		self
	}
	
	/// Offset the top border by the given amount.
	pub fn move_top_border(&mut self, by: f64) -> &mut Self {
		match self.vertical {
			AnchorFrom::Start(ref mut y) => *y += by,
			AnchorFrom::StartEnd(ref mut y, _) => *y += by,
			_ => {}
		}
		self
	}
	
	/// Offset the bottom border by the given amount.
	pub fn move_bottom_border(&mut self, by: f64) -> &mut Self {
		match self.vertical {
			AnchorFrom::End(ref mut y) => *y -= by,
			AnchorFrom::StartEnd(_, ref mut y) => *y -= by,
			_ => {}
		}
		self
	}
	
	
	/*pub fn left_margin(self, left_margin: f64) -> Anchors {
		match self.horizontal {
			Start(mut ref anchor_to) => { anchor_to.start_margin(left_margin); },
			StartEnd(mut ref anchor_to, _) => { anchor_to.start_margin(left_margin); },
		}
		self
	}*/
	
	/// Apply the anchor configuration to the given ItemData ApplyContext.
	pub fn apply(&self, item_data: &ItemData, cxt: &mut ApplyContext<ItemDataChange>) -> &Self {
		let mut rect = Rect::new(item_data.pos, item_data.size);
		
		match self.horizontal {
			AnchorFrom::None => {},
			AnchorFrom::Start(left) => {
				rect.set_left(left);
			},
			AnchorFrom::End(right) => {
				rect.set_right(right);
			},
			AnchorFrom::StartEnd(left, right) => {
				rect.set_left(left);
				rect.set_right(right);
			},
			AnchorFrom::Middle(middle) => {
				let half_width = rect.size.x / 2.;
				rect.pos.x = middle - half_width;
			},
		}
		
		match self.vertical {
			AnchorFrom::None => {},
			AnchorFrom::Start(top) => {
				rect.set_top(top);
			},
			AnchorFrom::End(bottom) => {
				rect.set_bottom(bottom);
			},
			AnchorFrom::StartEnd(top, bottom) => {
				rect.set_top(top);
				rect.set_bottom(bottom);
			},
			AnchorFrom::Middle(middle) => {
				let half_height = rect.size.y / 2.;
				rect.pos.y = middle - half_height;
			},
		}
		
		if item_data.pos != rect.pos {
			cxt.apply(ItemDataChange::pos(ValueChange(rect.pos)));
		}
		
		if item_data.size != rect.size {
			cxt.apply(ItemDataChange::size(ValueChange(rect.size)));
			//let ah = cxt.apply_handle();
			//ah.invoke(ItemDataChange::size(ValueChange(rect.size)));
		}
		
		self
	}
}
