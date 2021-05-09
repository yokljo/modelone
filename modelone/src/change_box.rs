use crate::model::{Change, Changeable, Revertable, Watcher, SubWatcher, apply_pipe_to_mut_ref};

use std;
use std::borrow::BorrowMut;

impl<C: Change> Change for Box<C> {
	type SignalType = Box<C::SignalType>;
	
	/*fn get_signals(&self) -> Vec<Self::SignalType> {
		self.as_ref().get_signals().into_iter().map(|signal| Box::new(signal)).collect()
	}*/
}

/// Applying Box<C> to T will just apply C to T
impl<T: Changeable<C> + std::cmp::PartialEq, C: Change> Changeable<Box<C>> for T {
	fn changeable_apply(&mut self, change: Box<C>, watcher: &mut Watcher<Box<C::SignalType>>) {
		let mut watcher_fn = |signal| {
			watcher.send_signal(Box::new(signal));
		};
		self.changeable_apply(*change, &mut SubWatcher::new(&mut watcher_fn));
	}
	
	fn reset_view_signals(&self) -> Vec<Box<C::SignalType>> {
		let mut signals = vec![];
		for signal in self.reset_view_signals() {
			signals.push(Box::new(signal));
		}
		signals
	}
}

/// Applying Box<C> to T will just apply C to T
impl<T: Revertable<C> + std::cmp::PartialEq, C: Change> Revertable<Box<C>> for T {
	fn revertable_apply(&mut self, mut change: Box<C>, watcher: &mut Watcher<Box<C::SignalType>>) -> Box<C> {
		let mut watcher_fn = |signal| {
			watcher.send_signal(Box::new(signal));
		};
		//Box::new(self.revertable_apply(*change))
		apply_pipe_to_mut_ref(|change| self.revertable_apply(change, &mut SubWatcher::new(&mut watcher_fn)), change.borrow_mut());
		change
	}
}
