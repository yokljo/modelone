new_items!{
	item Button {
		// This would be better if there was a pass through mode for properties where bindings to
		// them forward changes and signals to and from sub items' properties. That way, the main
		// struct would not have the field, but there would still be a valid signal/change for it in
		// the signal/change enums.
		prop text: Arc<String>;
		signal clicked: ();
		prop other_names: Vec<String>[VecChange<String>];
		
		// This is a signal to the parser itself that it should make this a mouse item.
		config is_mouse_item: true;
		// "on" expects a ValueChange? Or maybe it only works for signals.
		/*on mouse.contains_mouse {
			if ~mouse.contains_mouse {
				back_rect.border_colour = 
			}
		}*/
		on mouse.pressed {
			if ~mouse.pressed != MouseButtons::None {
				~apply(~root.clicked(()));
			}
		}
		
		// This matches against the signal, unlike the one above which just ignores the signal value
		on match other_names {
			VecSignal::Set{index} => println!("{}", index),
			_ => {},
		}
		
		// Maybe to find the Change and Signal, it will just append "Change" and "Signal" to the
		// name. That way if you want some specific combination, you can just put them in a module
		// with the appropriate names. Then here, you would refer to it as my_rect::Rectangle, and
		// then it will look for my_rect::RectangleChange.
		Rectangle(back_rect) {
			anchors.fill: ~parent;
			colour: if ~root.mouse.pressed != MouseButtons::None { Colour::rgb(0.5, 0.5, 0.5) } else { Colour::rgb(0.5, 0.5, 0.5) };
			border_colour: if ~root.mouse.contains_mouse { std_colour::LIME } else { std_colour::RED };
			
			Text(label) {
				// "anchors" is basically a keyword here, where the parser gathers all the
				// "anchors.blah" declarations and generates an Anchors instance.
				anchors.fill: ~parent;
				anchors.margins: 5.;
				text: ~root.text;
			}
		}
	}
}

new_items!{
	struct Button {
		prop text: Arc<String>;
		signal clicked: ();
		prop other_names: Vec<String>[VecChange<String>];
		
		config is_mouse_item: true;
		on mouse.pressed {
			if ~mouse.pressed != MouseButtons::None {
				~apply(~root.clicked(()));
			}
		}
		
		on match other_names {
			VecSignal::Set{index} => println!("{}", index),
			_ => {},
		}
		
		Rectangle(back_rect) {
			anchors.fill: ~parent;
			colour: if ~root.mouse.pressed != MouseButtons::None { Colour::rgb(0.5, 0.5, 0.5) } else { Colour::rgb(0.5, 0.5, 0.5) };
			border_colour: if ~root.mouse.contains_mouse { std_colour::LIME } else { std_colour::RED };
			
			Text(label) {
				anchors.fill: ~parent;
				anchors.margins: 5.;
				text: ~root.text;
			}
		}
	}
}

struct Button {
	item_data: ItemData,
	
	text: Arc<String>,
	clicked: JustSignal,
	other_names: Vec<String>
	
	back_rect: Rectangle,
	label: Text,
}

impl_changeable_struct!{ButtonChange[ButtonSignal] for Button:
	item_data: ItemDataChange,
	
	text: ValueChange<Arc<String>>,
	clicked: JustSignalChange<()>,
	other_names: VecChange<String>,
	
	back_rect: RectangleChange,
	label: TextChange,
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
				// It would take some smarts to combine all relevant signals like this
				cxt.apply(ButtonChange::clicked(JustSignalChange(())));
			}
		}
		
		if let ButtonSignal::other_names(ref sub_signal) = *signal {
			match sub_signal {
				VecSignal::Set{index} => println!("{}", index),
				_ => {},
			}
		}
		
		dispatch_struct_update!{ButtonChange[ButtonSignal] for self, cxt, signal:
			item_data: ItemData,
			back_rect: Rectangle,
			label: Text,
		}
	}
}

/////////////////////////////////////////

new_items!{
	// The "root" is the CheckBox itself. The root can't inherit other items, it can only contain
	// them.
	struct CheckBox {
		let name: String[StringChange] = "Hello World";
		// No change type implies ValueChange<bool>
		let checked: bool = false;
		
		Rectangle {
			// Somehow this has to generate a Anchors binding to the parent
			
			//parent.fill: true
			
			// OR
			
			impl {
				// All "parent" references here would be replaced with "cxt.get()", and "self"
				// replaced with "sub_apply!(cxt, CheckBoxChange::item_1.RectangleChange::item_data)"
				Anchors::new_fill(AnchorRelation::Parent, parent)
					.apply(self);
			}
			
			Label {
				// text must also use String[StringChange]
				text = root.name;
			}
		}
	}
	
	struct TreeViewRow {
		
	}
	
	struct TreeView {
		let model:
	
		Repeater {
			model = root.model
		}
	}
}

// Generated code:

struct CheckBox {
	item_data: ItemData,
	name: String,
	item_1: Rectangle,
	item_2: Label,
}

impl_changeable_struct!{CheckBoxChange[CheckBoxSignal] for CheckBox:
	item_data: ItemDataChange,
	name: StringChange,
	item_1: RectangleChange,
	item_2: LabelChange,
}

impl Item for CheckBox {
	impl_get_item!(item_data);
	
	impl_children!{
		item_1 {
			item_2
		},
	}
}

impl Object<CheckBox, CheckBoxChange> for CheckBox {
	fn update(cxt: &mut ApplyContext<CheckBox, CheckBoxChange>, signal: &CheckBoxSignal) {
		Anchors::new_fill(AnchorRelation::Parent, cxt.get())
			.apply(sub_apply!(cxt, CheckBoxChange::item_1.RectangleChange::item_data));

		if let CheckBoxSignal::name(field_signal) = signal {
		// How do I get a change from a signal?
			cxt.apply(field_signal);
		}
			
		dispatch_struct_update!{CheckBoxChange[CheckBoxSignal] for signal, cxt:
			item_data: ItemData,
			item_1: Rectangle,
			item_2: Label,
		}
	}
}

fn main() {
    println!("Hello, world!");
}
