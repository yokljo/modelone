# Modelone

This is a project of mine that went perhaps a little too far. It has the following parts:

## Model
I designed a novel model system in Rust with built-in undo support. Basically, you make two accompanying types:
- A "change" type, which would usually be an enum with all the different types of operations you can perform on a type:
```rust
#[derive(Debug, Clone, PartialEq)]
enum VecChange<T: Changeable<C>, C: Change> {
	/// Set the value at `index` to `item`
	Set{index: usize, item: T},
	/// Insert `item` at `index`
	Insert{index: usize, item: T},
	/// Remove the item from `index`
	Remove{index: usize},
	/// Replace the entire vector with a different one
	ReplaceAll(Vec<T>),
	/// Apply the given `change` to the item at `index`
	At{index: usize, change: C}
}
```
- A "signal" type, which is basically the same as the change type, but with only enough information to know where data changed:
```rust
enum VecSignal<ST> {
	/// Set the value at `index` to `item`
	Set{index: usize},
	/// Insert `item` at `index`
	Insert{index: usize},
	/// Remove the item from `index`
	Remove{index: usize},
	/// Replace the entire vector with a different one
	ReplaceAll,
	/// Apply the given `change` to the item at `index`
	At{index: usize, signal: ST}
}
```

For a basic struct, the change type would just be an enum with an entry for each field, with the associated change type for that field:
```rust
struct People {
	names: Vec<String>,
	average_age: f32,
}
enum PeopleChange {
	// VecChange allows you to add or remove elements to/from the names list, or go inside one of the elements and apply a StringChange, which allows you to add/remove ranges of characters to/from the string.
	Names(VecChange<StringChange>),
	// ValueChange only allows you to swap out the entire value of a field.
	AverageAge(ValueChange<f32>),
}
enum PeopleSignal {
	Names(VecSignal<StringSignal>),
	AverageAge(ValueSignal),
}
```

The enums can be generated automatically by a macro.

To apply a change, you do something like this:
```rust
// People implements the Changeable<PeopleChange> trait.
let people = People {
	names: vec!["joe", "brent"],
	average_age: 41.2,
};

// This is a debugger used to collect signals produced by changeable_apply into a list:
let mut watcher = SpyWatcher::new();

// changeable_apply is a method of the Changeable trait.
people.changeable_apply(
	// Change the names field of the People struct:
	PeopleChange::Names(
		// Change index 1 of the names list:
		VecChange::At{
			index: 1,
			change: StringChange {
				// Change "brent" to "bill":
				index: 1,
				len: 4,
				new: "ill".to_string(),
			},
		}
	),
	// This argument must implement the Watcher trait (for example, SpyWatcher). When changeable_apply "emits" a signal, it calls the send_signal method on this argument:
	&mut watcher,
);

assert_eq!(people, People {
	names: vec!["joe", "bill"],
	average_age: 41.2,
});
assert_eq!(watcher.signals, vec![
	PeopleSignal::Names(
		VecSignal::At{
			index: 1,
			signal: StringSignal {
				index: 1,
				from_len: 4,
				to_len: 3,
			},
		}
	),
]);
```

Then, anything can use the Watcher trait's send_signal method to do stuff specific to the exact modification that was made.

Then I made undo possible. For this to work, the People struct implements the `Revertable` trait in addition to the `Changeable` trait. `Revertable` gives you a new method: `revertable_apply`. It does almost the same thing as `changeable_apply`, except that it must return another change that, when applied, will return the struct to its original state:
```rust
let people = People {
	names: vec!["joe", "brent"],
	average_age: 41.2,
};

let mut watcher = SpyWatcher::new();

// This does the same thing as changeable_apply:
let revert_change = people.revertable_apply(
	// This is the same change as above:
	PeopleChange::Names(
		VecChange::At{
			index: 1,
			change: StringChange {
				index: 1,
				len: 4,
				new: "ill".to_string(),
			},
		}
	),
	&mut watcher,
);

// Look, it's the same as before:
assert_eq!(people, People {
	names: vec!["joe", "bill"],
	average_age: 41.2,
});

// But now, I can undo it!
people.changeable_apply(
	revert_change,
	&mut watcher,
);

assert_eq!(people, People {
	names: vec!["joe", "brent"],
	average_age: 41.2,
});
assert_eq!(watcher.signals, vec![
	// Same as before:
	PeopleSignal::Names(
		VecSignal::At{
			index: 1,
			signal: StringSignal {
				index: 1,
				from_len: 4,
				to_len: 3,
			},
		}
	),
	// This is the signal produced by applying revert_change:
	PeopleSignal::Names(
		VecSignal::At{
			index: 1,
			signal: StringSignal {
				index: 1,
				// Note here how the lengths are swapped:
				from_len: 3,
				to_len: 4,
			},
		}
	),
]);
```
## Object system

I wanted to be able to keep a reference to another part of the same model. For example:
```rust
// This model is a tree of Nodes:
struct Node {
	name: String,
	size: f32,
	children: Vec<Node>,
	// Any node can keep a reference to any other node:
	reference: Option<some_kind_of_thing_that_permanently_refers_to_another_node>,
}

// For example:
let root = Node {
	name: "Root".to_string(),
	size: 6.2,
	children: vec![
		// I want to keep a reference to this node from the "Joe" node:
		Node {
			name: "Bob".to_string(),
			size: 2.4,
			children: vec![],
			reference: None,
		},
		Node {
			name: "Joe".to_string(),
			size: 10000.0,
			children: vec![],
			reference: Some(a_reference_to_Bob),
		},
	],
	reference: None,
};
```

A goal of this project was to be able to keep the model as "plain old data": You can implement Changeable on literally any struct you like and add change signalling functionality to it without modifying the original struct.

This is where it got a little out-of-control...

The way I achieved it was with a thing called a ChangeConstructor, which is basically a description of how to get to anywhere in the model, from the root of the model. That description can be used to construct a change that can be applied directly to the root of the model to change a speicific part of the model, or to get the value of a specific part of the model via the root.

In this specific example to modify Bob, we would need:
```rust
NodeChange::Children(
	VecChange::At{
		index: 0,
		change: a_change_to_Bob,
	}
)
```

Note how `a_change_to_Bob` doesn't exist until you actually want to change Bob, and therefore the entire value is impossible to construct. That's what ChangeConstructor is for. It is basically a nested structure representing the path down to the Bob node.

Then, every type must implement the ChangeConstructor trait. Here's the one for Vec:
```rust
pub struct VecChangeConstructor<C: Change> {
	index: usize,
	sub: Box<ChangeConstructor<C>>,
}

impl<T: 'static + Changeable<C>, C: Change> ChangeConstructor<VecChange<T, C>> for VecChangeConstructor<C> {
	fn create(&self, leaf_change: Box<std::any::Any>) -> VecChange<T, C> {
		VecChange::At{
			index: self.index,
			change: self.sub.create(leaf_change)
		}
	}
	
	// I'll get to this in a sec...
	fn update(&mut self, change: &VecChange<T, C>) -> bool {
		use self::VecChange::*;
		match *change {
			Set{index, ..} if index == self.index => {
				false
			}
			Insert{index, ..} if index <= self.index => {
				self.index += 1;
				true
			}
			Remove{index} => {
				if self.index == index {
					false
				} else if self.index > index {
					self.index -= 1;
					true
				} else {
					true
				}
			}
			ReplaceAll{..} => {
				false
			}
			At{index, ref change} if index == self.index => {
				self.sub.update(change)
			}
			_ => true
		}
	}
}
```

When you have a ChangeConstructor that describes how to make a change to Bob, you can do this:
```rust
// Change Bob's size to 1234.5:
let size_change_for_bob = bob_change_constructor.create(Box::new(NodeChange::Size(ValueChange(1234.5))));
// If you call `create()` with anything other than NodeChange (the type of Bob), it will panic :)

// Actually make the change.
root.changeable_apply(size_change_for_bob);
```

Okay, so now the problem is that the references might become invalid when you change the model. For example, the Bob reference refers to index 0 of the children list of the root. What happens then if you add another child before Bob in the root's children list?

That's where the `update` function in the ChangeConstructor trait comes in. For every change you make to the root, you have to call update on all existing change constructors (this can be optimised a little bit) with that change. The update function will consider what each change is modifying, and update the change constructor if it needs to. So in the example where you insert another child before Bob, the update function on Vec will recognise `VecChange::Insert{index: 0, ...}` and update Bob's ChangeConstructor to point to index 1 instead. The update function can also say that a ChangeConstructor is now invalid, for example, if a change were to remove the Bob node entirely.

So how do you update all the ChangeConstructors? Well, where I wrote `a_reference_to_Bob` in the first code sample above, I store an int, which is an index into a global array of ChangeConstructors. Then, the root of the model must be stored in a thing called a `Manager`. The manager keeps track of the root of the model, and the list of Change Constructors. If you want to apply a change to the model, you must ask the Manager to do it for you. The model will make sure all appropriate ChangeConstructors are updated for each change that comes through.

## GUI

I then had a crazy idea: What if I could use this model as the basis for a GUI library? The entire widget tree would be stored in a nested structure with the Changeable trait implemented everywhere.

This included a change to the model to add a thing called an `ApplyHandle`. You can make an instance of an ApplyHandle out of a ChangeConstructor, then pass it around wherever you like, including passing it to another thread. You can then use the ApplyHandle to add model changes to a queue in the Manager, which can then be synchronously passed into the associated ChangeConstructor and applied to the model. The ChangeConstructor will always be up-to-date because it can be updated even after using `ApplyHandle` to queue a change).

To implement signals like "on click" or whatever, I made a thing called `JustSignalChange(T)`, which when applied, emits the value T as a signal, and applies no real changes to the model.

I don't think I'll bother to explain how `ApplyContext` works right now. It's the thing that allows widgets to handle events locally and make changes to the model even though they are deeply nested within the root widget's struct.

A Button widget might look like this:

```rust
pub struct Button {
	pub item_data: ItemData,
	pub back_rect: Rectangle,
	pub label: Text,
}

pub struct ButtonChange {
	ItemData(ItemDataChange),
	BackRect(RectangleChange),
	Label(TextChange),
	// Note that this doesn't have an associated field in the Button struct:
	OnClick(JustSignalChange<f32>),
}

pub struct ButtonSignal {
	ItemData(ItemDataSignal),
	BackRect(RectangleSignal),
	Label(TextSignal),
	// Note that this doesn't have an associated field in the Button struct:
	OnClick(f32),
}

let button = Button {...};
let mut watcher = SpyWatcher::new();
button.changeable_apply(ButtonChange::OnClick(JustSignalChange(5.)), &mut watcher);
assert_eq!(watcher.signals, vec![ButtonSignal::OnClick(5.)]);
```

I did actually manage to make a somewhat working UI with a complicated resource/graphics system.

![UI screenshot](ui_screenshot.png)
*I know I know, the font system doesn't do sub-pixel antialiasing.*

The problem is that with large UI models with deep nesting, the change enums grow to be extremely large, and require multiple levels of boxing for every change you want to make. It then passes around these large nested enums by value, which would also get arbitrarily expensive. Maybe with a bump allocator and some other tricks it could be sped up. For now, I don't think this idea will ever pan out.


## GUI macro language

Once I had the GUI system basically working, I had the idea that I could make a strictly typed language akin to QML that gets executed inside a macro, and automatically generates the Changeable implementations for all the types required from the macro invocation:

```rust
// This generates Button, ButtonChange and ButtonSignal structs.
#[item_builder] item!(Button {
	prop text: Arc<String>;
	prop tooltip_text: Arc<String> = #{text}.clone();
	
	Label {
		prop cool: String[StringChange];
		text = #{root:cool} + #{cool} + "!";
	}
});
```

I don't think it currently compiles, but I got it kinda working at one point.

The code behind the proc macro is scary.

© 2021 Joshua Worth
