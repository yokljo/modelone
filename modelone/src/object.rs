use crate::model::*;
use crate::idalloc::IdAlloc;

use std;
use std::mem;
use std::sync::{mpsc, Mutex, Condvar, Arc};
use std::any::Any;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Handle(usize);

#[derive(Debug, Clone, PartialEq)]
pub struct JustSignal;

/// The most basic of changes, which just swaps the value with another
#[derive(Debug, Clone, PartialEq)]
pub struct JustSignalChange<ST>(pub ST);

impl<ST: 'static> Change for JustSignalChange<ST> where
	ST: std::cmp::PartialEq + Clone + std::fmt::Debug + Send
{
	type SignalType = ST;
}

impl<ST: 'static> Changeable<JustSignalChange<ST>> for JustSignal where
	ST: std::cmp::PartialEq + Clone + std::fmt::Debug + Send
{
	fn changeable_apply(&mut self, change: JustSignalChange<ST>, watcher: &mut Watcher<ST>) {
		watcher.send_signal(change.0);
	}
	
	fn reset_view_signals(&self) -> Vec<ST> {
		vec![]
	}
}

/*struct Task<ST> {
	handle: Handle,
	name: String,
	cancellable: bool,
	cancelling: bool,
	progress: f64,
	progress_max: f64,
	signal: JustSignal<String>,
}

impl_changeable_struct!{TaskChange<ST>[TaskSignal<ST>] for Task<ST>:
	Name name: ValueChange<String>,
	Cancellable cancellable: ValueChange<bool>,
	Cancelling cancelling: ValueChange<bool>,
	Progress progress: ValueChange<f64>,
	ProgressMax progress_max: ValueChange<f64>,
	Signal signal: JustSignalChange<ST>,
}*/

#[derive(Debug)]
pub enum ApplyHandleMessage {
	/// Apply the given change (wrapped in Any) using the ChangeConstructor with the ID.
	Apply(usize, Box<Any + Send>),
	/// Apply the given changes (wrapped in Any) all at once using the ChangeConstructor with the ID.
	ApplyAll(usize, Vec<Box<Any + Send>>),
	/// Indicate that an apply handle was cloned for the ChangeConstructor with the given ID, so
	/// increment its reference count.
	Clone(usize),
	/// Indicate that an apply handle was dropped for the ChangeConstructor with the given ID, so
	/// decrement its reference count.
	Drop(usize),
}

pub struct ApplyHandleAny {
	id: usize,
	async_change_queue_send: mpsc::Sender<ApplyHandleMessage>,
	async_change_notifier: AsyncChangeNotifier,
}

impl ApplyHandleAny {
	fn new(id: usize, async_change_queue_send: mpsc::Sender<ApplyHandleMessage>, async_change_notifier: AsyncChangeNotifier) -> ApplyHandleAny {
		ApplyHandleAny { id, async_change_queue_send, async_change_notifier }
	}
	
	/// Returns false if the invocation fails (when the model has already been deleted).
	pub fn invoke<C: 'static + Send>(&self, change: C) -> bool {
		let result = self.async_change_queue_send.send(ApplyHandleMessage::Apply(self.id, Box::new(change) as Box<Any + Send>));
		self.async_change_notifier.notify();
		result.is_err()
	}
	
	/// Returns false if the invocation fails (when the model has already been deleted).
	pub fn invoke_all<C: 'static + Send>(&self, changes: Vec<C>) -> bool {
		let changes_any = changes.into_iter().map(|change| Box::new(change) as Box<Any + Send>).collect();
		let result = self.async_change_queue_send.send(ApplyHandleMessage::ApplyAll(self.id, changes_any));
		self.async_change_notifier.notify();
		result.is_err()
	}
}

impl Clone for ApplyHandleAny {
	fn clone(&self) -> ApplyHandleAny {
		self.async_change_queue_send.send(ApplyHandleMessage::Clone(self.id)).ok();
		
		ApplyHandleAny {
			id: self.id,
			async_change_queue_send: self.async_change_queue_send.clone(),
			async_change_notifier: self.async_change_notifier.clone(),
		}
	}
}

impl Drop for ApplyHandleAny {
	fn drop(&mut self) {
		self.async_change_queue_send.send(ApplyHandleMessage::Drop(self.id)).ok();
	}
}

#[derive(Clone)]
pub struct ApplyHandle<C> {
	apply_handle_any: ApplyHandleAny,
	phantom: std::marker::PhantomData<C>,
}

impl<C: 'static + Send> ApplyHandle<C> {
	fn new(apply_handle_any: ApplyHandleAny) -> ApplyHandle<C> {
		ApplyHandle {
			apply_handle_any,
			phantom: std::marker::PhantomData,
		}
	}
	
	pub fn invoke(&self, change: C) {
		self.apply_handle_any.invoke(change);
	}
}

/*struct TransactionScope<T: Changeable<C>, C: Change> {
	apply_context: Option<&ApplyContext<T, C>>
}*/

/// An ApplyContext is something that can be used to apply changes to a model.
pub trait ApplyContext<C: Change> {
	/// Apply the given change to the model.
	fn apply(&mut self, change: C);
	
	// TODO: This mechanism doesn't account for ignoring nested transactions!
	/// Make a new transaction if this is part of an revertable model.
	fn new_transaction(&mut self, name: String);
	
	/// Create an ApplyHandleAny that should use the given constructor to build a change.
	fn apply_handle_any(&mut self, constructor: Box<ChangeConstructor<C>>) -> ApplyHandleAny;
	
	/// Creates an apply handle that can be used to asynchronously apply changes using the same
	/// context as this ApplyContext. This uses apply_handle_any, and does does not need to be
	/// implemented manually.
	fn apply_handle(&mut self) -> ApplyHandle<C> {
		let any_handle = self.apply_handle_any(Box::new(LeafChangeConstructor::new()));
		ApplyHandle::new(any_handle)
	}
}

/*struct ModelView<Model, ModelChange, View, ViewChange> {
	model: &mut ApplyContext<Model, ModelChange>,
	view: &mut ApplyContext<View, ViewChange>,
}

enum ModelViewSignal<ModelSignal, ViewSignal> {
	Model(ModelSignal),
	View(ViewSignal),
}*/

pub trait Object<C: Change> {
	fn update(&self, _cxt: &mut ApplyContext<C>, _signal: &C::SignalType) {}
	//fn event(cxt: &mut ApplyType, event: EventType) {}
}

/// This macro is used to dispatch signals to their appropriate sub-structs.
///
/// # Example
///
/// ```ignore
/// # #[macro_use] extern crate modelone;
/// #[macro_use] use modelone::object;
/// dispatch_struct_update!{AppUiChange[AppUiSignal] for self, cxt, signal:
/// 	item_data: ItemData,
/// 	input_field: TextField,
/// }
/// ```
#[macro_export] macro_rules! dispatch_struct_update{
	($change_name:ident[$signal_name:ident] for $model:expr, $cxt:tt, $signal:tt:
		$($field_name:ident: $type:ty,)+
	) => {
		#[allow(unreachable_patterns)]
		match *$signal {
			$(
				$signal_name::$field_name(ref subsignal) => {
					$model.$field_name.update(
						&mut $crate::object::SubApplyContext::new(
							$cxt,
							&mut |change| $change_name::$field_name(change),
							//&|| Box::new(|change| $change_name::$field_name(change))
							&|sub_constructor| Box::new($crate::model::SubChangeConstructor::new(
								sub_constructor,
								//fn new(create_fn: fn(&std::any::Any) -> C, update_fn: fn(&C) -> bool) -> FnChangeConstructor<C> {
								|sub_constructor, leaf_change| {
									let sub_change = sub_constructor.create(leaf_change);
									$change_name::$field_name(sub_change)
								},
								|sub_constructor, change| {
									#[allow(unreachable_patterns)]
									match *change {
										$change_name::$field_name(ref sub_change) => {
											sub_constructor.update(sub_change)
										}
										_ => { true }
									}
								},
								|sub_constructor| format!("{}/{}", stringify!($field_name), sub_constructor.debug_string())
							))
						),
						subsignal
					);
				}
			)*
			_ => {}
		}
	};
	// This allows for not including a final trailing comma
	($change_name:ident[$signal_name:ident] for $model:expr, $cxt:tt, $signal:tt:
		$($field_name:ident: $type:ty),+
	) => (
		dispatch_struct_update!{$model, $change_name[$signal_name] for $model, $cxt, $signal:
			$($field_name: $type,)*
		}
	)
}

// Any acceptable Changeable type can be passed as an ApplyContext
// without being wrapped in anything else first.
/*impl<T: Changeable<C>, C> ApplyContext<T, C> for T {
	fn get(&self) -> &T {
		self
	}
	
	fn apply(&mut self, change: C) {
		self.changeable_apply(change);
	}
	
	fn new_transaction(&mut self, _: String) {
		// This ApplyContext is not undoable, so just ignore new transactions
	}
}*/

pub trait Validator<C: Change> {
	#[must_use]
	fn validate(&mut self, cxt: &mut ApplyContext<C>, changes: &Vec<C>) -> Result<(), String>;
}

pub struct NoValidator;
impl<C: Change> Validator<C> for NoValidator {
	fn validate(&mut self, _cxt: &mut ApplyContext<C>, _changes: &Vec<C>) -> Result<(), String> { Ok(()) }
}

/*struct ValidatorApplier<'t, T: 't+Changeable<C>, C: 't+Change> {
	manager_data: &'t mut ManagerData<T, C>,
}

impl<'t, T: Changeable<C>, C: Change+std::fmt::Debug+std::clone::Clone> ApplyContext<T, C> for ValidatorApplier<'t, T, C> {
	fn get(&self) -> &T {
		&self.manager_data.model
	}
	
	fn apply(&mut self, change: C) {
		let signal_queue = &mut self.manager_data.signal_queue;
		let mut watcher_fn = |signal| {
			signal_queue.push(signal);
		};
		self.manager_data.model.changeable_apply(change.clone(), &mut SubWatcher::new(&mut watcher_fn));
		//self.manager_data.change_queue.push(change);
	}
	
	fn apply_handle_any(&self, constructor: Box<ChangeConstructor<C>>) -> ApplyHandleAny {
		//ApplyHandleAny::new(0, self.)
	}
	
	fn new_transaction(&mut self, name: String) {
		// Ignore new transactions during validation
	}
	
	/*fn get_handle() -> Handle {
		Handle(0)
	}*/
}*/

struct AsyncChangeNotifierInternal {
	mutex: Mutex<()>,
	condvar: Condvar,
}

#[derive(Clone)]
pub struct AsyncChangeNotifier {
	internal: Arc<AsyncChangeNotifierInternal>,
}

impl AsyncChangeNotifier {
	fn new() -> AsyncChangeNotifier {
		AsyncChangeNotifier {
			internal: Arc::new(AsyncChangeNotifierInternal {
				mutex: Mutex::new(()),
				condvar: Condvar::new(),
			})
		}
	}

	pub fn notify(&self) {
		self.internal.condvar.notify_all();
	}
	
	pub fn wait(&self) {
		let _dummy_mutex_guard = self.internal.condvar.wait(self.internal.mutex.lock().unwrap()).unwrap();
	}
}

type ChangeConstructorSet<C> = IdAlloc<(Option<Box<ChangeConstructor<C>>>, usize)>;

/// Internal data for Manager.
struct ManagerData<T: Changeable<C>, C: Change> {
	/// The managed model.
	model: T,
	/// Signals yet to be sent to the view.
	signal_queue: Vec<C::SignalType>,
	//handles: HashMap<Handle, C::SignalType>,
}

impl<T, C> ManagerData<T, C> where
	T: Changeable<C>,
	C: 'static + Change + std::fmt::Debug + std::clone::Clone,
{
	fn apply_change(&mut self, change: C, change_constructors: &mut ChangeConstructorSet<C>) {
		let signal_queue = &mut self.signal_queue;
		let mut watcher_fn = |signal| {
			signal_queue.push(signal);
		};
		
		self.model.changeable_apply(change.clone(), &mut SubWatcher::new(&mut watcher_fn));
		
		change_constructors.apply_to_all_mut(&mut |_, &mut (ref mut opt_change_constructor, _)| {
			let still_valid = if let Some(ref mut change_constructor) = *opt_change_constructor {
				change_constructor.update(&change)
			} else {
				true
			};
			
			if !still_valid {
				*opt_change_constructor = None;
			}
		});
	}
	
	fn process_apply_handle_message(&mut self, message: ApplyHandleMessage, change_constructors: &mut ChangeConstructorSet<C>) {
		match message {
			ApplyHandleMessage::Apply(id, any_change) => {
				let mut opt_change = None;
				
				if let (Some(ref change_constructor), _) = *change_constructors.get(id) {
					opt_change = Some(change_constructor.create(any_change));
				} else {
					println!("Invalidated apply handle was called");
				}
				
				if let Some(change) = opt_change {
					//println!("Applying {:?}: {:?}", id, change);
					self.apply_change(change, change_constructors);
				}
			}
			ApplyHandleMessage::ApplyAll(id, any_changes) => {
				any_changes.into_iter().for_each(|any_change| {
					let mut opt_change = None;
					
					if let (Some(ref change_constructor), _) = *change_constructors.get(id) {
						opt_change = Some(change_constructor.create(any_change));
					} else {
						println!("Invalidated apply handle was called");
					}
					
					if let Some(change) = opt_change {
						//println!("Applying {:?}: {:?}", id, change);
						self.apply_change(change, change_constructors);
					}
				});
			}
			ApplyHandleMessage::Clone(id) => {
				println!("Clone {}", id);
				let (_, ref mut ref_count) = *change_constructors.get_mut(id);
				*ref_count += 1;
			}
			ApplyHandleMessage::Drop(id) => {
				println!("Drop {:?}", id);
				let mut should_dealloc = false;
				{
					let (_, ref mut ref_count) = *change_constructors.get_mut(id);
					*ref_count -= 1;
					
					if *ref_count == 0 {
						should_dealloc = true;
					}
				}
				
				if should_dealloc {
					println!("Dealloc {:?}", id);
					change_constructors.deallocate(id);
				}
			}
		}
	}
	
	fn consume_all(&mut self, changes: &mut Vec<C>, change_constructors: &mut ChangeConstructorSet<C>) {
		for change in changes.drain(..) {
			self.apply_change(change, change_constructors);
		}
	}
	
}

struct ChangeQueue<C: Change> {
	changes: Vec<C>,
	/// Asynchronous change queue.
	async_change_queue_recv: mpsc::Receiver<ApplyHandleMessage>,
	async_change_queue_send: mpsc::Sender<ApplyHandleMessage>,
	async_change_notifier: AsyncChangeNotifier,
	/// ID allocator for ChangeConstructors. If the option is None, then the constructor became
	/// invalid and was removed. The usize is the reference count.
	change_constructors: ChangeConstructorSet<C>,
}

impl<C: Change> ChangeQueue<C> {
	fn new() -> ChangeQueue<C> {
		let (async_change_queue_send, async_change_queue_recv) = mpsc::channel();

		ChangeQueue {
			changes: vec![],
			async_change_queue_send,
			async_change_queue_recv,
			async_change_notifier: AsyncChangeNotifier::new(),
			change_constructors: ChangeConstructorSet::new(),
		}
	}
}

impl<C> ApplyContext<C> for ChangeQueue<C> where
	C: 'static + Change + std::fmt::Debug + std::clone::Clone,
{
	fn apply(&mut self, change: C) {
		self.changes.push(change);
	}
	
	fn new_transaction(&mut self, _name: String) {
		// There is no undoing in managers, so ignore new transaction requests
	}
	
	fn apply_handle_any(&mut self, constructor: Box<ChangeConstructor<C>>) -> ApplyHandleAny {
		let debug_string = constructor.debug_string();
		let (_, id) = self.change_constructors.allocate((Some(constructor), 1));
		println!("Creating apply handle({}): /{}", id, debug_string);
		let new_send = self.async_change_queue_send.clone();
		let async_change_notifier = self.async_change_notifier.clone();
		/*Box::new(move |leaf_change| {
			new_send.send();
			async_change_notifier.notify();
		})*/
		ApplyHandleAny::new(id, new_send, async_change_notifier)
	}
	
	/*fn apply_handle(&self) -> Box<Fn(C) + Send> {
		let new_send = self.async_change_queue_send.clone();
		let async_change_notifier = self.async_change_notifier.clone();
		Box::new(move |change| {
			println!("Change: {:?}", change);
			new_send.send(change).unwrap();
			async_change_notifier.notify();
		})
	}*/
	
	/*fn get_handle() -> Handle {
		Handle(0)
	}*/
}

// To modify the model or listen to modifications of the model, you need access
// to the model's manager.
pub struct Manager<T: Changeable<C>, C: Change, V: Validator<C>> {
	// This data is separated so that it can be passed to the validator
	data: ManagerData<T, C>,
	change_queue: ChangeQueue<C>,
	// While the validator is in use, the other stuff still needs to be
	// appliable. Solution: make a ValidatedManager and a non-validated Manager
	#[allow(dead_code)]
	validator: V,
}

impl<T, C, V> Manager<T, C, V> where
	T: Changeable<C> + Object<C>,
	C: 'static + Change + std::fmt::Debug + std::clone::Clone,
	V: Validator<C>,
{
	pub fn new(model: T, validator: V) -> Manager<T, C, V> {
		Manager {
			data: ManagerData {
				model,
				signal_queue: vec![],
				//handles: HashMap::new(),
			},
			change_queue: ChangeQueue::new(),
			validator,
		}
	}
	
	pub fn get(&self) -> &T {
		&self.data.model
	}
	
	pub fn apply(&mut self, change: C) {
		self.change_queue.apply(change);
		self.apply_all_queued();
	}
	
	pub fn apply_all_queued(&mut self) {
		self.data.consume_all(&mut self.change_queue.changes, &mut self.change_queue.change_constructors);
	}
	
	/// This destroys the manager forever, returning the internal model as value
	pub fn take_model(self) -> T {
		self.data.model
	}
	
	/// Gets a list of all signals that need to be emitted to refresh the state
	/// of a view watching this manager.
	pub fn reset_view_signals(&self) -> Vec<C::SignalType> {
		self.data.model.reset_view_signals()
	}
	
	pub fn reset_view(&mut self) {
		for signal in self.reset_view_signals() {
			self.data.model.update(&mut self.change_queue, &signal);
			self.apply_all_queued();
		}
	}
	
	/*pub fn apply_option(&mut self, optional_change: Option<C>) {
		if let Some(change) = optional_change {
			self.apply(change);
		}
	}*/
	
	pub fn process_async_changes(&mut self) {
		for message in self.change_queue.async_change_queue_recv.iter() {
			self.data.process_apply_handle_message(message, &mut self.change_queue.change_constructors);
			self.data.consume_all(&mut self.change_queue.changes, &mut self.change_queue.change_constructors);
		}
	}
	
	pub fn try_process_async_changes(&mut self) {
		for message in self.change_queue.async_change_queue_recv.try_iter() {
			self.data.process_apply_handle_message(message, &mut self.change_queue.change_constructors);
			self.data.consume_all(&mut self.change_queue.changes, &mut self.change_queue.change_constructors);
		}
	}
	
	pub fn get_async_change_notifier(&self) -> AsyncChangeNotifier {
		self.change_queue.async_change_notifier.clone()
	}
	
	/// Updates the view with all queued signals.
	pub fn resolve_signals(&mut self) {
		loop {
			let signal_queue = self.take_signal_queue();
			if signal_queue.len() == 0 {
				break;
			}
			
			for signal in signal_queue {
				//println!("Signal: {:?}", signal);
				self.data.model.update(&mut self.change_queue, &signal);
				self.apply_all_queued();
			}
			
		}
	}
	
	/// Returns the list of currently queued signals and empties the internal
	/// queue.
	pub fn take_signal_queue(&mut self) -> Vec<C::SignalType> {
		mem::replace(&mut self.data.signal_queue, vec![])
	}
	
	//fn validate_changes(&mut self, _changes: &Vec<C>) {
		// TODO: Make the validator function run after a transaction has
		// completed, and allow the validator to fail, causing it to undo all
		// changes in that transaction and discard them
		/*let mut applier = ValidatorApplier {
			manager_data: &mut self.data,
		};
		self.validator.validate(&mut applier, changes);*/
	//}
}

/*struct Message<'t, 'c, T: Revertable<C>+'t, C: 'c> {
	data: &'t T,
	change: &'c C,
}

trait View<T: Revertable<C>, C> {
	fn dispatch(message: Message<T, C>) {
		
	}
}*/

// 'p is the parent's lifetime, 'c is the child-item's lifetime. T is for the
// represented model type, C is for the type T has Changeable implemented
// for. PT and PC are the respective equivalent types for the parent's context.
pub struct SubApplyContext<'p, PC: 'static + Change, C: 'static + Change> {
	parent_context: &'p mut ApplyContext<PC>,
	wrap_fn: &'p Fn(C) -> PC,
	//box_wrap_fn: &'p Fn() -> Box<Fn(C) -> PC + Send>,
	wrap_constructor_fn: &'p Fn(Box<ChangeConstructor<C>>) -> Box<ChangeConstructor<PC>>,
	//apply_handle_fn: &'p Fn(C) -> Box<Fn(PC)>,
	//handle_fn: &'p Fn(C) -> Handle,
}

impl<'p, PC: 'static + Change, C: 'static + Change> SubApplyContext<'p, PC, C> {
	pub fn new(
			parent_context: &'p mut ApplyContext<PC>,
			wrap_fn: &'p Fn(C) -> PC,
			//box_wrap_fn: &'p Fn() -> Box<Fn(C) -> PC + Send>,
			wrap_constructor_fn: &'p Fn(Box<ChangeConstructor<C>>) -> Box<ChangeConstructor<PC>>,
	) -> SubApplyContext<'p, PC, C> {
		SubApplyContext {
			parent_context,
			wrap_fn,
			//box_wrap_fn,
			wrap_constructor_fn,
		}
	}
}

impl<'p, PC: 'static + Change, C: 'static + Change> ApplyContext<C> for SubApplyContext<'p, PC, C> {
	fn apply(&mut self, change: C) {
		self.parent_context.apply((self.wrap_fn)(change));
	}
	
	fn apply_handle_any(&mut self, constructor: Box<ChangeConstructor<C>>) -> ApplyHandleAny {
		let parent_constructor = (self.wrap_constructor_fn)(constructor);
		self.parent_context.apply_handle_any(parent_constructor)
	}
	
	/*fn apply_handle(&self) -> Box<Fn(C) + Send> {
		let parent_handle = self.parent_context.apply_handle();
		let wrap_fn = (self.box_wrap_fn)();
	
		Box::new(move |change| {
			parent_handle(wrap_fn(change));
		})
	}*/
	
	fn new_transaction(&mut self, _name: String) {
		// Because this SubApplyContext consumes changes, it can't undo,
		// therefore new transactions are ignored
	}
}

/// eg. `sub_apply!(cxt, AppUiChange::title_field.TextFieldChange::item_data)`
#[macro_export] macro_rules! sub_apply {
	(@impl ($($change_name:tt)*) $change_type:ident::$field_name:ident $($sub_change_type:ident::$sub_field_name:ident)+) => {
		$change_type::$field_name(sub_apply!(@impl ($($change_name)*) $($sub_change_type::$sub_field_name)*))
	};
	(@impl ($($change_name:tt)*) $change_type:ident::$field_name:ident) => {
		$change_type::$field_name($($change_name)*)
	};
	
	($cxt:ident, $change_type:ident::$field_name:ident $(.$sub_change_type:ident::$sub_field_name:ident)*) => {
		&mut $crate::object::SubApplyContext::new(
			$cxt,
			&|change| sub_apply!(@impl (change) $change_type::$field_name $($sub_change_type::$sub_field_name)*),
			//&|| Box::new(|change| sub_apply!(@impl change $change_type::$field_name $($sub_change_type::$sub_field_name)*))
			&|sub_constructor| Box::new($crate::model::SubChangeConstructor::new(
				sub_constructor,
				//fn new(create_fn: fn(&std::any::Any) -> C, update_fn: fn(&C) -> bool) -> FnChangeConstructor<C> {
				|sub_constructor, leaf_change| {
					let sub_change = sub_constructor.create(leaf_change);
					//$change_type::$field_name(sub_change)
					sub_apply!(@impl (sub_change) $change_type::$field_name $($sub_change_type::$sub_field_name)*)
				},
				|sub_constructor, change| {
					//if let $change_type::$field_name(ref sub_change) = change {
					if let sub_apply!(@impl (ref sub_change) $change_type::$field_name $($sub_change_type::$sub_field_name)*) = *change {
						sub_constructor.update(sub_change)
					} else {
						true
					}
				},
				|sub_constructor| format!("{}/{}", stringify!($field_name).to_string() $(+ "/" + stringify!($sub_field_name))*, sub_constructor.debug_string())
			))
		)
	};
}

pub struct RevertableSubApplyContext<'p, 'c, PC: 'static + Change, C: 'static + Change> {
	parent_context: &'p mut ApplyContext<PC>,
	wrap_fn: &'c Fn(C) -> PC,
	//box_wrap_fn: &'p Fn() -> Box<Fn(C) -> PC>,
	wrap_constructor_fn: &'p Fn(Box<ChangeConstructor<C>>) -> Box<ChangeConstructor<PC>>,
	new_transaction_fn: &'c mut Fn(String),
}

impl<'p, 'c, PC: 'static + Change, C: 'static + Change> RevertableSubApplyContext<'p, 'c, PC, C> {
	pub fn new(
			parent_context: &'p mut ApplyContext<PC>,
			wrap_fn: &'c Fn(C) -> PC,
			//box_wrap_fn: &'p Fn() -> Box<Fn(C) -> PC>,
			wrap_constructor_fn: &'p Fn(Box<ChangeConstructor<C>>) -> Box<ChangeConstructor<PC>>,
			new_transaction_fn: &'c mut Fn(String)
	) -> RevertableSubApplyContext<'p, 'c, PC, C> {
		RevertableSubApplyContext {
			parent_context,
			wrap_fn,
			//box_wrap_fn,
			wrap_constructor_fn,
			new_transaction_fn,
		}
	}
}

impl<'p, 'c, PC: 'static + Change, C: 'static + Change> ApplyContext<C> for RevertableSubApplyContext<'p, 'c, PC, C> {
	fn apply(&mut self, change: C) {
		self.parent_context.apply((self.wrap_fn)(change));
	}
	
	fn apply_handle_any(&mut self, constructor: Box<ChangeConstructor<C>>) -> ApplyHandleAny {
		let parent_constructor = (self.wrap_constructor_fn)(constructor);
		self.parent_context.apply_handle_any(parent_constructor)
	}
	
	/*fn apply_handle(&self) -> Box<Fn(C) + Send> {
		//let parent_handle = self.parent_context.apply_handle();
	
		Box::new(move |change| {
			//parent_handle((self.apply_handle_fn)(change));
		})
	}*/
	
	fn new_transaction(&mut self, name: String) {
		(self.new_transaction_fn)(name);
	}
}

/*trait View {
	fn dispatch(Revertable) -> 
}*/
