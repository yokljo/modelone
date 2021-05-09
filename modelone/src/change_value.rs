use crate::model::{Change, Changeable, Revertable, Watcher};

use std;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueSignal;

/// The most basic of changes, which just swaps the value with another
#[derive(Debug, Clone, PartialEq)]
pub struct ValueChange<T: std::cmp::PartialEq>(pub T);

impl<T: 'static + std::cmp::PartialEq + Send> Change for ValueChange<T> {
	type SignalType = ValueSignal;
	
	/*fn get_signals(&self) -> Vec<Self::SignalType> {
		vec![ValueSignal]
	}*/
}

/// The most basic of changes, which just swaps the value with another of the
/// same type. This means a type can be used as the change type for itself.
impl<T: 'static + std::cmp::PartialEq + Send> Revertable<ValueChange<T>> for T {
	fn revertable_apply(&mut self, mut change: ValueChange<T>, watcher: &mut Watcher<ValueSignal>) -> ValueChange<T> {
		if *self != change.0 {
			std::mem::swap(self, &mut change.0);
			watcher.send_signal(ValueSignal);
		}
		change
	}
}

impl<T: 'static + std::cmp::PartialEq + Send> Changeable<ValueChange<T>> for T {
	fn changeable_apply(&mut self, change: ValueChange<T>, watcher: &mut Watcher<ValueSignal>) {
		if *self != change.0 {
			*self = change.0;
			watcher.send_signal(ValueSignal);
		}
	}
	
	fn reset_view_signals(&self) -> Vec<ValueSignal> {
		vec![ValueSignal]
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use model::{SubChangeConstructor, LeafChangeConstructor, SpyWatcher};
	
	#[test] fn value_change() {
		let mut spy = SpyWatcher::new();
	
		let mut v = 6;
		let revert = v.revertable_apply(ValueChange::Set(8), &mut spy);
		assert_eq!(v, 8);
		v.revertable_apply(revert, &mut spy);
		assert_eq!(v, 6);
		v.changeable_apply(ValueChange::Set(8), &mut spy);
		assert_eq!(v, 8);
		
		assert_eq!(spy.signals, vec![ValueSignal, ValueSignal, ValueSignal]);
	}
}
