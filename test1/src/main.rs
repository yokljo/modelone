#[macro_use] extern crate modelone;
#[macro_use] extern crate uione;

use modelone::object::*;
use modelone::history::*;
use modelone::change_string::*;
use modelone::change_vec::*;
use modelone::change_value::*;

use uione::vec2::*;
use uione::*;

use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
struct NameRecord {
	first_name: String,
	last_name: String,
}

impl_revertable_struct!{NameRecordChange[NameRecordSignal] for NameRecord:
	first_name: StringChange,
	last_name: StringChange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Editor {
	editor_title: String,
	names: Vec<String>,
	name_ref: Option<usize>,
	sub_editors: Vec<Editor>,
	user_name: NameRecord,
}

impl_revertable_struct!{EditorChange[EditorSignal] for Editor:
	editor_title: StringChange,
	names: ValueVecChange<String>,
	name_ref: ValueChange<Option<usize>>,
	sub_editors: Box<VecChange<Editor, EditorChange>>,
	user_name: NameRecordChange,
}

/*struct EditorValidator;

impl Validator<Editor, EditorChange> for EditorValidator {
	fn validate(&mut self, cxt: &mut ApplyContext<Editor, EditorChange>, changes: &Vec<EditorChange>) -> Result<(), String> {
		for change in changes {
			if let EditorChange::names(ref names_change) = *change {
				let new_ref = names_change.updated_reference(self.name_ref);
				cxt.apply(EditorChange::name_ref(ValueChange::Set(new_ref)));
			}
		}
		Ok(())
	}
}*/

#[derive(Debug, Clone, PartialEq)]
struct AppModel {
	title: String,
	editor: History<Editor, EditorChange>,
}

impl_changeable_struct!{AppModelChange[AppModelSignal] for AppModel:
	title: StringChange,
	editor: HistoryChange<Editor, EditorChange>,
}

// AppModelChange::wrap_title

/*struct AppModelValidator;

impl Validator<AppModel, AppModelChange> for AppModelValidator {
	fn validate(&mut self, cxt: &mut ApplyContext<AppModel, AppModelChange>, changes: &Vec<AppModelChange>) -> Result<(), String> {
		for change in changes {
			match *change {
				AppModelChange::DoNothing => {},
				AppModelChange::title(_) => {
					if self.title == "My App 2" {
						//cxt.apply(AppModelChange::Title(ValueChange("My App 3".into())));
					}
				},
				AppModelChange::editor(ref _editorchange) => {
					//let subcxt = &mut SubApplyContext::new(cxt, &mut |c| ModelChange::Tags(c))
					//EditorValidator{}.validate(subcxt, change.);
				},
			}
		}
		Ok(())
	}
}*/

#[derive(Debug, PartialEq)]
struct AppUi {
	item_data: ItemData,
	title_field: TestWidget1,
	editor_title_field: TestWidget1,
	app_model: AppModel,
	my_button: Button,
}

impl_changeable_struct!{AppUiChange[AppUiSignal] for AppUi:
	item_data: ItemDataChange,
	title_field: TestWidget1Change,
	editor_title_field: TestWidget1Change,
	app_model: AppModelChange,
	my_button: ButtonChange,
}

/*#[derive(Debug, PartialEq, Clone)]
enum EnumThing {
	S(String),
	V(i32),
}

impl_revertable_enum!{EnumThingChange[EnumThingSignal] for EnumThing:
	S: StringChange,
	V: ValueChange<i32>,
}*/

impl AppUi {
	fn new(app_model: AppModel) -> AppUi {
		let mut my_button = Button::new(Arc::new("Press me".into()));
		my_button.item_data.size.x = 100. * 10.;
		my_button.item_data.size.y = 25. * 5.;
	
		AppUi {
			item_data: ItemData::new(),
			title_field: TestWidget1::new(),
			editor_title_field: TestWidget1::new(),
			app_model: app_model,
			my_button: my_button,
		}
	}
}

impl Object<AppUiChange> for AppUi {
	fn update(&self, cxt: &mut ApplyContext<AppUiChange>, signal: &AppUiSignal) {
		Anchors::new_fill_margin(AnchorRelation::Parent, self, 10.)
			.apply(&self.title_field.item_data, sub_apply!(cxt, AppUiChange::title_field.TestWidget1Change::item_data));
		
		if let AppUiSignal::my_button(ButtonSignal::clicked(())) = *signal {
			println!("Clicked!");
		}
		
		dispatch_struct_update!{AppUiChange[AppUiSignal] for self, cxt, signal:
			item_data: ItemData,
			title_field: TestWidget1,
			my_button: Button,
			//editor_title_field: TestWidget1,
		}
	}
}

impl Item for AppUi {
	impl_get_item!(item_data);

	impl_children!{
		title_field,
		my_button,
		//editor_title_field,
	}
}

impl AppUi {
	//fn event(_cxt: &mut ApplyContext<AppUi, AppUiChange>, _event: UiEvent) {
		//for signal in self.title_field.event(consume_apply_wrap!(model, AppModelChange, Title title), event) {
		//for signal in TestWidget1::event(&mut SubApplyContext::new(cxt, &|model| &model.title_field, &mut |change| AppUiChange::title_field(change)), event) {
			// TODO
		//}
	//}
	
	/*fn event_from_model(cxt: &mut ApplyContext<AppUi, AppUiChange>, model: &AppModel, signal: &AppModelSignal) {
		match *signal {
			AppModelSignal::title(ref _subsignal) => {
				if model.title != self.title_field.content.model.text {
					//TestWidget1::replace_text(&mut SubApplyContext::new(cxt, &|model| &model.title, &mut |change| AppModelChange::Title(change)), self.title_field);
				}
			},
			AppModelSignal::editor(ref subsignal) => {
				match *subsignal {
					HistorySignal::Change(ref subsubsignal) => {
						match *subsubsignal {
							EditorSignal::editor_title(ref _editortitlesignal) => {
								//cxt.apply(AppUiChange::EditorTitleField(TestWidget1Change::
								//self.editor_title_field.update(&model.editor.model.editor_title, editortitlesignal);
								//TestWidget1::replace_text(&mut SubApplyContext::new(cxt, &|model| &model.editor_title_field, &mut |change| AppUiChange::editor_title_field(change)), model.editor.model.editor_title.clone());
								TestWidget1::replace_text(sub_apply!(cxt, AppUiChange::editor_title_field), model.editor.model.editor_title.clone());
							},
							_ => {},
						}
					},
					HistorySignal::Reset => {
						let changeable: &Changeable<StringChange> = &model.editor.model.editor_title;
						for signal in changeable.reset_view_signals() {
							println!("Reset Editor Signal: {:?}", signal);
							//self.editor_title_field.update(&model.editor.model.editor_title, &signal);
						}
					},
					_ => {},
				}
			},
		}
	}*/
}

fn main() {
	let app_model = AppModel {
		title: "My App".into(),
		editor: History::new(Editor {
			editor_title: "Blah".into(),
			names: vec!["Hello World".into(), "Cool Beans".into()],
			name_ref: Some(1),
			sub_editors: vec![],
			user_name: NameRecord { first_name: "Joe".into(), last_name: "Bloggs".into() },
		})
	};
	
	let mut ui = AppUi::new(app_model);
	ui.item_data.size = Vec2f::new(800., 600.);
	let mut manager = Manager::<_, AppUiChange, _>::new(ui, NoValidator);
	
	/*for signal in manager.reset_view_signals() {
		println!("Reset Signal: {:?}", signal);
		AppUi::update(&mut manager, &signal);
	}*/
	
	/*for change in manager.get().reset_view_changes() {
		ui.update(manager.get(), change);
	}*/
	//manager.resolve_signals();
	
	//println!("{:#?}", manager.get());
	//println!("{:#?}", manager.get());
	
	manager.apply(AppUiChange::app_model(AppModelChange::editor(HistoryChange::Push(EditorChange::editor_title(StringChange{index: 0, len: 0, new: "a".into()})))));
	//ui.event(&mut manager, UiEvent::Text("a".into()));
	//manager.apply(AppModelChange::Editor(HistoryChange::Push(EditorChange::EditorTitle(StringChange{index: 0, len: 0, new: "a".into()}))));
	
	//manager.resolve_signals();
	
	//println!("{:#?}", manager.get());
	
	//manager.apply(AppModelChange::Editor(HistoryChange::Undo));
	//manager.resolve_signals();
	
	println!("{:#?}", manager.get());
	
	//let scenegraph = SceneGraph::new();
	
	println!("Change size: {}", std::mem::size_of::<AppModelChange>());
	
	//mainloop_gfx_sdl::exec(&mut manager, get_root_item_change);
	mainloop_gl_glutin::exec(manager, get_root_item_change, &|| {
		vec![
			Box::new(font_resource::FontResource::new())
		]
	});
	//println!("{:05X}", 50);
}

fn get_root_item_change(item_change: ItemDataChange) -> AppUiChange {
	AppUiChange::item_data(item_change)
}
