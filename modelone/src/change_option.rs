use crate::model::{Change, Changeable, Revertable, Watcher, SubWatcher, ChangeConstructor};

use std;

/// A change for Option types.
/*#[derive(Debug, Clone)]
pub enum OptionChange<T: Revertable<C>, C> {
	Set(Option<T>),
	Change(C),
}

impl<T: Revertable<C>, C> Revertable<OptionChange<T, C>> for Option<T> {
	fn revertable_apply(&mut self, change: OptionChange<T, C>) -> OptionChange<T, C> {
		use self::OptionChange::*;
		match change {
			Set(mut item) => {
				std::mem::swap(self, &mut item);
				Set(item)
			},
			Change(subchange) => {
				let revertchange = self.as_ref().unwrap().revertable_apply(subchange);
				Change(revertchange)
			},
		}
	}
}*/

#[derive(Debug, Clone, PartialEq)]
pub enum OptionSignal<ST> {
	/// The Option's entire value changed.
	Reset,
	/// The value in the Some variant of the Option changed.
	Change(ST),
}

/// A change for Option types, which supports nested changes.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionChange<T: Changeable<C>, C: Change> {
	/// Set the Option's entire value.
	Reset(Option<T>),
	/// Apply the given change to the value in the some variant of the Option.
	Change(C),
}

impl<T: 'static + Changeable<C> + Send, C: Change> Change for OptionChange<T, C> {
	type SignalType = OptionSignal<C::SignalType>;
}

impl<T: 'static + Changeable<C> + Send, C: Change> Changeable<OptionChange<T, C>> for Option<T> {
	fn changeable_apply(&mut self, change: OptionChange<T, C>, watcher: &mut Watcher<OptionSignal<C::SignalType>>) {
		use self::OptionChange::*;
		match change {
			Reset(value) => {
				*self = value;
				watcher.send_signal(OptionSignal::Reset);
			}
			Change(subchange) => {
				let mut watcher_fn = |signal| {
					watcher.send_signal(OptionSignal::Change(signal));
				};
				self.as_mut().unwrap().changeable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn));
			}
		}
	}
	
	fn reset_view_signals(&self) -> Vec<OptionSignal<C::SignalType>> {
		vec![OptionSignal::Reset]
	}
}

impl<T: 'static + Revertable<C> + Send, C: Change> Revertable<OptionChange<T, C>> for Option<T> {
	fn revertable_apply(&mut self, change: OptionChange<T, C>, watcher: &mut Watcher<OptionSignal<C::SignalType>>) -> OptionChange<T, C> {
		use self::OptionChange::*;
		match change {
			Reset(mut value) => {
				std::mem::swap(self, &mut value);
				watcher.send_signal(OptionSignal::Reset);
				Reset(value)
			}
			Change(subchange) => {
				let mut watcher_fn = |signal| {
					watcher.send_signal(OptionSignal::Change(signal));
				};
				let revertchange = self.as_mut().unwrap().revertable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn));
				Change(revertchange)
			}
		}
	}
}

pub struct OptionChangeConstructor<C: Change> {
	sub: Box<ChangeConstructor<C>>,
}

impl<C: Change> OptionChangeConstructor<C> {
	pub fn new(sub: Box<ChangeConstructor<C>>) -> OptionChangeConstructor<C> {
		OptionChangeConstructor{sub}
	}
}

impl<T: 'static + Changeable<C>, C: Change> ChangeConstructor<OptionChange<T, C>> for OptionChangeConstructor<C> {
	fn create(&self, leaf_change: Box<std::any::Any>) -> OptionChange<T, C> {
		OptionChange::Change(self.sub.create(leaf_change))
	}
	
	fn update(&mut self, change: &OptionChange<T, C>) -> bool {
		use self::OptionChange::*;
		match *change {
			Reset(..) => {
				false
			}
			Change(ref change) => {
				self.sub.update(change)
			}
		}
	}
	
	fn debug_string(&self) -> String {
		format!("Some/{}", self.sub.debug_string())
	}
}
