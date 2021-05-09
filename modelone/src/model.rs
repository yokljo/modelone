use std;
use std::mem;
use std::process;
use std::any::Any;

/// A trait for things than can watch for signals sent when applying a Change.
pub trait Watcher<ST> {
	fn send_signal(&mut self, signal: ST);
}

/// A no-op implementation of Watcher.
pub struct NoWatcher;
impl<ST> Watcher<ST> for NoWatcher {
	fn send_signal(&mut self, _signal: ST) {
		// Do nothing
	}
}

/// A simple lambda-based watcher which can be used to listen to signals
/// directly, or propagate them to another signal watcher.
pub struct SubWatcher<'p, ST: 'p> {
	watcher_fn: &'p mut FnMut(ST),
}

impl<'p, ST> SubWatcher<'p, ST> {
	pub fn new(watcher_fn: &'p mut FnMut(ST)) -> SubWatcher<'p, ST> {
		SubWatcher {
			watcher_fn,
		}
	}
}

impl<'p, ST> Watcher<ST> for SubWatcher<'p, ST> {
	fn send_signal(&mut self, signal: ST) {
		(self.watcher_fn)(signal);
	}
}

/// A watcher which collects signals in a Vec. Useful for debugging.
pub struct SpyWatcher<ST> {
	pub signals: Vec<ST>,
}

impl<ST> SpyWatcher<ST> {
	pub fn new() -> SpyWatcher<ST> {
		SpyWatcher {
			signals: vec![],
		}
	}
}

impl<ST> Watcher<ST> for SpyWatcher<ST> {
	fn send_signal(&mut self, signal: ST) {
		self.signals.push(signal);
	}
}

/// A trait for types that work as changes for other types.
pub trait Change: Send + Sized + Any {
	// A change always has a signal type to go with it.
	type SignalType: std::fmt::Debug + Clone + PartialEq;
	// All changes come with a method of
	//type ConstructorType: ChangeConstructor<Self>;

	//fn get_signals(&self) -> Vec<Self::SignalType> { vec![] }
}

/// Implement this on a type VT to allow VT to be updated with the data in CT.
/// For example, assume a type StringChange exists which describes how to modify
/// a String. That desription can be applied to the string by calling
/// `my_string.changeable_apply(change)` if Changeable is implemented
/// with `impl Changeable<StringChange> for String`.
pub trait Changeable<CT: Change>: Send {
	/// This is the same as apply, except it doesn't return a change for undoing
	/// the change. This can be implemented to be faster than revertable_apply,
	/// but the simplest implementation would be an alias for revertable_apply
	fn changeable_apply(&mut self, change: CT, watcher: &mut Watcher<CT::SignalType>);
	
	/// This should be implemented to return a vector of changes which should
	/// fully reset all data in a view on the implemented change. For example,
	/// for a vector `[VecSignal::ReplaceAll]` should be returned. Changes need
	/// to be able to have signals that fully reset a value regardless of the
	/// value's original data, so for example types such as StringChange say
	/// that the length to remove is usize::MAX, so it will always replace
	/// everything.
	fn reset_view_signals(&self) -> Vec<CT::SignalType>;
}

/// This extends on Changeable if you can take any instance of the given
/// change type CT and create a new instance which does the inverse operation
/// so that if an instance of CT is applied to a value, another instance can
/// always be created that when applied puts the value back in exactly the same
/// state it originated. In other words, implement this on things that can be
/// changed, then reverted again.
pub trait Revertable<CT: Change>: Changeable<CT> {
	/// This applies the given `change` to self, then returns a change that
	/// can be used to revert it
	fn revertable_apply(&mut self, change: CT, watcher: &mut Watcher<CT::SignalType>) -> CT;
}

/// Implement this on a type that describes how to build a change that is nested inside another
/// change. The nested change will be passed as an argument to `create`, which will then wrap up
/// that change inside a change of type `C`.
pub trait ChangeConstructor<C: Change> {
	/// Creates a change, propagating the leaf_change down the constructor chain until it reaches an
	/// end point, when it will completely resolve the change.
	fn create(&self, leaf_change: Box<std::any::Any>) -> C;
	
	/// Updates the ChangeConstructor, returns true if the thing the change applied to still exists.
	fn update<'s, 'c>(&'s mut self, change: &'c C) -> bool;
	
	fn debug_string(&self) -> String;
}

/// This is the end point ChangeConstructor that unwraps the leaf_change.
pub struct LeafChangeConstructor<C: Change> {
	phantom: std::marker::PhantomData<C>,
}

impl<C: Change> LeafChangeConstructor<C> {
	pub fn new() -> LeafChangeConstructor<C> {
		LeafChangeConstructor {
			phantom: std::marker::PhantomData,
		}
	}
}

impl<C: 'static + Change> ChangeConstructor<C> for LeafChangeConstructor<C> {
	fn create(&self, leaf_change: Box<std::any::Any>) -> C {
		if let Ok(leafchange_concrete) = leaf_change.downcast::<C>() {
			*leafchange_concrete
		} else {
			panic!("Any did not have the correct type");
		}
	}
	
	fn update(&mut self, _change: &C) -> bool {
		true
	}
	
	fn debug_string(&self) -> String {
		"?".into()
	}
}

pub struct SubChangeConstructor<PC: Change, C: Change> {
	sub_constructor: Box<ChangeConstructor<C>>,
	create_fn: fn(&Box<ChangeConstructor<C>>, Box<std::any::Any>) -> PC,
	update_fn: fn(&mut Box<ChangeConstructor<C>>, &PC) -> bool,
	debug_string_fn: fn(&Box<ChangeConstructor<C>>) -> String,
}

impl<PC: Change, C: Change> SubChangeConstructor<PC, C> {
	pub fn new(
		sub_constructor: Box<ChangeConstructor<C>>,
		create_fn: fn(&Box<ChangeConstructor<C>>, Box<std::any::Any>) -> PC,
		update_fn: fn(&mut Box<ChangeConstructor<C>>, &PC) -> bool,
		debug_string_fn: fn(&Box<ChangeConstructor<C>>) -> String,
	) -> SubChangeConstructor<PC, C> {
		SubChangeConstructor { sub_constructor, create_fn, update_fn, debug_string_fn }
	}
}

impl<PC: Change, C: Change> ChangeConstructor<PC> for SubChangeConstructor<PC, C> {
	fn create(&self, leafchange: Box<std::any::Any>) -> PC {
		(self.create_fn)(&self.sub_constructor, leafchange)
	}
	
	fn update<'s, 'c>(&'s mut self, change: &'c PC) -> bool {
		(self.update_fn)(&mut self.sub_constructor, change)
	}
	
	fn debug_string(&self) -> String {
		(self.debug_string_fn)(&self.sub_constructor)
	}
}

/// A macro for generating all the boilerplate enums for Revertable types
/*macro_rules! impl_changeable{
	($change_name:ident for $model_name:ident as $self_decl:tt:
		$($entry_name:ident $change_type:tt: ($($arg:pat),*) => $block:expr,)+
	) => {
		#[derive(Debug, Clone)]
		enum $change_name {
			$($entry_name $change_type,)*
		}
		
		impl model::Revertable<$change_name> for $model_name {
			fn revertable_apply(&mut $self_decl, change: $change_name) -> $change_name {
				use $change_name::*;
				match change {
					$($entry_name($($arg,)*) => $entry_name($block),)*
				}
			}
		}
		
		impl model::Changeable<$change_name> for $model_name {
			fn changeable_apply(&mut $self_decl, change: $change_name) {
				$self_decl.revertable_apply(change);
			}
		}
	};
	// This allows for not including a final trailing comma
	($change_name:ident for $model_name:ident as $self_decl:tt:
		$($entry_name:ident $change_type:tt: ($($arg:pat),*) => $block:expr),+
	) => (
		impl_changeable!{$change_name for $model_name as $self_decl:
			$($entry_name $change_type: ($($arg),*) => $block,)*
		}
	)
}

macro_rules! macro_id{
	(($tok:tt)*) => {$($tok)*}
}*/

#[macro_export] macro_rules! impl_changeable_body{
	(on_changed $model_name:ident (&mut $arg:ident) $body:expr) => {
		fn on_changed($arg: &mut $model_name) {
			$body
		}
	};
}

//$(do $action_name => $action:expr,)* 
#[macro_export] macro_rules! impl_changeable_struct{
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$(on_reset => ($($reset_arg:tt)*) $reset_body:expr,)*
		$($field_name:ident: $change_type:ty $(=> $mod:ident ($($arg:tt)*) $body:expr)*,)+
	) => {
		/*mod $change_name {
			$(
				type $field_name = $change_type;
			)*
		}*/
		
		#[allow(non_camel_case_types)]
		#[derive(Debug, Clone, PartialEq)]
		pub enum $signal_name {
 			$(
				$field_name(<$change_type as $crate::model::Change>::SignalType),
			)*
		}
		
		#[allow(non_camel_case_types)]
		#[derive(Debug, Clone, PartialEq)]
		pub enum $change_name {
			$($field_name($change_type),)*
		}
		
		impl $crate::model::Change for $change_name {
			type SignalType = $signal_name;
			
			/*fn get_signals(&self) -> Vec<Self::SignalType> {
				match *self {
					$($change_name::$field_name(ref subchange) => {
						subchange.get_signals().into_iter().map(|signal| $signal_name::$field_name(signal)).collect()
					},)*
				}
			}*/
		}
		
		impl $crate::model::Changeable<$change_name> for $model_name {
			fn changeable_apply(&mut self, change: $change_name, watcher: &mut $crate::model::Watcher<$signal_name>) {
				match change {
					$(
						$change_name::$field_name(subchange) => {
							let mut watcher_fn = |signal| {
								watcher.send_signal($signal_name::$field_name(signal));
							};
							
							$(
								$crate::impl_changeable_body!($mod $model_name ($($arg)*) $body);
								$mod(self);
							)*;
							
							self.$field_name.changeable_apply(subchange, &mut $crate::model::SubWatcher::new(&mut watcher_fn));
						},
					)*
				}
			}
			
			fn reset_view_signals(&self) -> Vec<$signal_name> {
				let mut signals = vec![];
				$(
					let changeable: &$crate::model::Changeable<$change_type> = &self.$field_name;
					for subsignal in changeable.reset_view_signals() {
						signals.push($signal_name::$field_name(subsignal));
					}
				)*
				signals
			}
		}
	};
	// This allows for not including a final trailing comma
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$($field_name:ident: $change_type:ty),+
	) => {
		impl_changeable_struct!{$change_name[$signal_name] for $model_name:
			$($field_name: $change_type,)*
		}
	};
}

#[macro_export] macro_rules! impl_revertable_struct{
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$($field_name:ident: $change_type:ty,)+
	) => {
		impl_changeable_struct!{$change_name[$signal_name] for $model_name:
			$($field_name: $change_type,)*
		}
		
		impl $crate::model::Revertable<$change_name> for $model_name {
			fn revertable_apply(&mut self, change: $change_name, watcher: &mut $crate::model::Watcher<$signal_name>) -> $change_name {
				match change {
					$(
						$change_name::$field_name(subchange) => {
							let mut watcher_fn = |signal| {
								watcher.send_signal($signal_name::$field_name(signal));
							};
							$change_name::$field_name(self.$field_name.revertable_apply(subchange, &mut $crate::model::SubWatcher::new(&mut watcher_fn)))
						},
					)*
				}
			}
		}
	};
	// This allows for not including a final trailing comma
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$($field_name:ident: $change_type:ty),+
	) => (
		impl_revertable_struct!{$change_name[$signal_name] for $model_name:
			$($field_name: $change_type,)*
		}
	)
}

/*macro_rules! impl_changeable_enum{
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$($variant_name:ident: $change_type:ty,)+
	) => {
		#[derive(Debug, Clone, PartialEq)]
		pub enum $signal_name {
			SetEnumValue,
 			$(
				$variant_name(<$change_type as Change>::SignalType),
			)*
		}
		
		#[derive(Debug, Clone, PartialEq)]
		pub enum $change_name {
			SetEnumValue($model_name),
			$($variant_name($change_type),)*
		}
		
		impl Change for $change_name {
			type SignalType = $signal_name;
		}
		
		impl Changeable<$change_name> for $model_name {
			fn changeable_apply(&mut self, change: $change_name, watcher: &mut Watcher<$signal_name>) {
				use $change_name::*;
				match change {
					$change_name::SetEnumValue(value) => {
						*self = value;
					},
					$(
						$change_name::$variant_name(subchange) => {
							if let $model_name::$variant_name(ref mut subvalue) = *self {
								let mut watcher_fn = |signal| {
									watcher.send_signal($signal_name::$variant_name(signal));
								};
								
								subvalue.changeable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn));
							} else {
								println!("Applied change to wrong enum variant");
							}
						},
					)*
				}
			}
			
			fn reset_view_signals(&self) -> Vec<$signal_name> {
				let mut signals = vec![$signal_name::SetEnumValue];
				signals
			}
		}
	};
	// This allows for not including a final trailing comma
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$($variant_name:ident: $change_type:ty),+
	) => {
		impl_changeable_enum!{$change_name[$signal_name] for $model_name:
			$($variant_name: $change_type,)*
		}
	};
}

macro_rules! impl_revertable_enum{
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$($variant_name:ident: $change_type:ty,)+
	) => {
		impl_changeable_enum!{$change_name[$signal_name] for $model_name:
			$($variant_name: $change_type,)*
		}
		
		impl Revertable<$change_name> for $model_name {
			fn revertable_apply(&mut self, change: $change_name, watcher: &mut Watcher<$signal_name>) -> $change_name {
				use $change_name::*;
				match change {
					$change_name::SetEnumValue(mut value) => {
						std::mem::swap(self, &mut value);
						$change_name::SetEnumValue(value)
					},
					$(
						$change_name::$variant_name(subchange) => {
							if let $model_name::$variant_name(ref mut subvalue) = *self {
								let mut watcher_fn = |signal| {
									watcher.send_signal($signal_name::$variant_name(signal));
								};
								
								subvalue.changeable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn));
							} else {
								println!("Applied change to wrong enum variant");
							}
						},
						
						$change_name::$variant_name(subchange) => {
							if let $model_name::$variant_name(ref mut subvalue) = *self {
								let mut watcher_fn = |signal| {
									watcher.send_signal($signal_name::$variant_name(signal));
								};
								$change_name::$variant_name(self.$variant_name.revertable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn)))
							} else {
								println!("Applied change to wrong enum variant");
							}
						},
					)*
				}
			}
		}
	};
	// This allows for not including a final trailing comma
	($change_name:ident[$signal_name:ident] for $model_name:ident:
		$($variant_name:ident: $change_type:ty),+
	) => (
		impl_revertable_enum!{$change_name[$signal_name] for $model_name:
			$($variant_name: $change_type,)*
		}
	)
}*/


/*#[derive(Debug, Clone)]
pub enum VecRefChange<'t, T: 't> {
	Set(Option<usize>),
	Update(&'t T),
}*/

/*pub struct TreeRow<T> {
	value: T,
	list: TreeList<T>,
}

pub type TreeList<T> = Vec<TreeRow<T>>;

pub enum TreeRowChange<T: Changeable<C>, C> {
	Set(T),
	Change(C),
	List(Box<VecChange<TreeRow<T>, TreeRowChange<T, C>>>),
}

impl<T: Revertable<C>, C> Revertable<TreeRowChange<T, C>> for TreeRow<T> {
	fn revertable_apply(&mut self, change: TreeRowChange<T, C>) -> TreeRowChange<T, C> {
		use self::TreeRowChange::*;
		match change {
			Set(mut value) => {
				std::mem::swap(&mut self.value, &mut value);
				Set(value)
			},
			Change(change) => {
				Change(self.value.revertable_apply(change))
			},
			List(list_change) => {
				List(Box::new(self.list.revertable_apply(*list_change)))
			},
		}
	}
}

impl<T: Changeable<C>, C> Changeable<TreeRowChange<T, C>> for TreeRow<T> {
	fn changeable_apply(&mut self, change: TreeRowChange<T, C>) {
		use self::TreeRowChange::*;
		match change {
			Set(value) => {
				self.value = value;
			},
			Change(change) => {
				self.value.changeable_apply(change);
			},
			List(list_change) => {
				self.list.changeable_apply(*list_change);
			},
		}
	}
}*/

/// Takes a function that takes a non-reference value of a type then returns a
/// value of the same type, then applies uses it to transform the value in the
/// given val_ref in-place.
///
/// # Examples
///
/// ```
/// fn add_thing(s: String) -> String {
/// 	s + " thing"
/// }
///
/// use modelone::model::apply_pipe_to_mut_ref;
///
/// let mut v = vec!["one".to_string(), "two".into()];
/// for elem in &mut v {
/// 	apply_pipe_to_mut_ref(add_thing, elem);
/// }
/// assert_eq!(v, vec!["one thing".to_string(), "two thing".into()]);
/// ```
pub fn apply_pipe_to_mut_ref<T, FT>(func: FT, val_ref: &mut T)
    where FT: FnOnce(T) -> T
{
	let val_copy: T = unsafe { mem::transmute_copy(val_ref) };
	
	struct ExitOnDrop;
	impl Drop for ExitOnDrop {
		fn drop(&mut self) {
			// If a panic happens in the function, the whole process must die
			// because func has already deleted val_copy at this point, so if it
			// keeps unrolling the same value will be deleted again when the
			// value pointed to by val_ref dies.
			process::exit(1);
		}
	}
	
	let exit_on_drop = ExitOnDrop;
	let mut val_new = func(val_copy);
	mem::forget(exit_on_drop);
	
	mem::swap(&mut val_new, val_ref);
	mem::forget(val_new);
}

#[cfg(test)]
mod tests {
	use super::*;
	use change_value::ValueChange;
	use change_string::StringChange;
	
	#[derive(Debug, Clone, PartialEq)]
	struct TestModel {
		first_name: String,
		last_name: String,
		age: u64,
	}

	impl_revertable_struct!{TestModelChange[TestModelSignal] for TestModel:
		first_name: StringChange,
		last_name: StringChange,
		age: ValueChange<u64>,
	}
	
	#[test] fn change_constructor() {
		{
			let lcc = LeafChangeConstructor::<ValueChange<u32>>::new();
			let change = lcc.create(Box::new(ValueChange::Set(123u32)));
			assert_eq!(change, ValueChange::Set(123u32));
		}
		
		{
			let lcc = Box::new(LeafChangeConstructor::<ValueChange<u64>>::new());
			
			let scc = SubChangeConstructor::new(lcc,
				|sub_constructor, any_change| {
					TestModelChange::age(sub_constructor.create(any_change))
				},
				|sub_constructor, change| {
					true
				},
				|_| "".into()
			);
			let change = scc.create(Box::new(ValueChange::Set(123u64)));
			assert_eq!(change, TestModelChange::age(ValueChange::Set(123u64)));
		}
	}
}
