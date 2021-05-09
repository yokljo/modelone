use uilang::item_builder;

use std::sync::Arc;
use modelone::change_string::StringChange;

/*#[item_builder]
item!(Stuff {
	prop text: Arc<String>;
});

#[item_builder]
item!(Thing {
	prop age: i32;
	// Perhaps child instances should be treated as normal properties that are automatically put
	// into a tree structure. Unnamed ones would still be properties, but unaccessible ones... Maybe
	// that isn't such a great idea.
	Stuff a {
		// There should be a way to somehow give a child a property relative to its parent, like in
		// QML with Layout.fillWidth: true
		child fill = true;
		prop blah: String[StringChange];
		//text: "cool beans";
	}
	Stuff {
	
	}
});*/

/*#[item_builder]
item!(Thing {
	prop age: i32;
	Stuff a {
		prop blah: String[StringChange];
	}
	Stuff {}
});*/

#[item_builder]
item!(Label {
	prop text: Arc<String>;
});

#[item_builder]
item!(Button {
	prop text: Arc<String>;
	//prop tooltip_text: Arc<String> = Arc::new("".into());
	prop tooltip_text: Arc<String> = #{text}.clone();
	prop cool: String[StringChange];
	
	Label {
		prop cool: String[StringChange];
		// If you go #{text} it will always refer to a local property. If you want a property from a
		// different instance, it needs to be stated like #{root.text} (the root item is always
		// called root)
		text = #{root:cool} + #{cool} + "!";
	}
});

fn main() {
	/*let s = Stuff {
		text: Arc::new("cool beans".to_string()),
	};
    println!("{}", s.text);*/
}
