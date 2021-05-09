use crate::model::{Change, Changeable, Revertable, Watcher, SubWatcher, ChangeConstructor};
use crate::change_value::{ValueChange, ValueSignal};

use std;

#[derive(Debug, Clone, PartialEq)]
pub enum VecSignal<ST> {
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

impl<ST> VecSignal<ST> {
	pub fn visit_changed_indices(&self, vec_size: usize, visitor: &Fn(usize)) {
		match *self {
			VecSignal::Set{index} => {
				visitor(index);
			}
			VecSignal::Insert{index} => {
				visitor(index);
			}
			VecSignal::Remove{index} => {
				visitor(index);
			}
			VecSignal::ReplaceAll => {
				for index in 0..vec_size {
					visitor(index);
				}
			}
			VecSignal::At{index, ..} => {
				visitor(index);
			}
		}
	}
}

/// A change for Vec types, which supports nested changes.
// TODO: This change could have a method which returns an iterator over all indices
// that have changed so listeners don't have to implement all of the possible
// changes
#[derive(Debug, Clone, PartialEq)]
pub enum VecChange<T: Changeable<C>, C: Change> {
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

impl<T: 'static + Changeable<C> + Send, C: Change> Change for VecChange<T, C> {
	type SignalType = VecSignal<C::SignalType>;
	
	/*fn get_signals(&self) -> Vec<Self::SignalType> {
		use VecChange::*;
		match *self {
			Set{index, ..} => vec![VecSignal::Set{index}],
			Insert{index, ..} => vec![VecSignal::Insert{index}],
			Remove{index, ..} => vec![VecSignal::Remove{index}],
			ReplaceAll(..) => vec![VecSignal::ReplaceAll],
			At{index, ref change} => change.get_signals().into_iter().map(|signal| VecSignal::At{index, signal}).collect(),
		}
	}*/
}

impl<T: Revertable<C>, C: Change> VecChange<T, C> {
	pub fn updated_reference(&self, index_reference: Option<usize>) -> Option<usize> {
		if let Some(ref_index) = index_reference {
			use self::VecChange::*;
			match *self {
				Insert{index, ..} => {
					if index <= ref_index {
						Some(ref_index + 1)
					} else {
						Some(ref_index)
					}
				},
				Remove{index} => {
					if index == ref_index {
						None
					} else if index < ref_index {
						Some(ref_index - 1)
					} else {
						Some(ref_index)
					}
				},
				_ => Some(ref_index)
			}
		} else {
			None
		}
	}
}

impl<T: 'static + Changeable<C> + Send, C: Change> Changeable<VecChange<T, C>> for Vec<T> {
	fn changeable_apply(&mut self, change: VecChange<T, C>, watcher: &mut Watcher<VecSignal<C::SignalType>>) {
		use self::VecChange::*;
		match change {
			Set{index, item} => {
				self[index] = item;
				/*watcher.tranform(|otherchange| {
					match otherchange {
						at@At{..} if at.index == index => {
							None
						},
						_ => otherchange
					}
				});*/
				watcher.send_signal(VecSignal::Set{index});
			},
			Insert{index, item} => {
				self.insert(index, item);
				watcher.send_signal(VecSignal::Insert{index});
			},
			Remove{index} => {
				self.remove(index);
				watcher.send_signal(VecSignal::Remove{index});
			},
			ReplaceAll(all) => {
				*self = all;
				watcher.send_signal(VecSignal::ReplaceAll);
			},
			At{index, change: subchange} => {
				let mut watcher_fn = |signal| {
					watcher.send_signal(VecSignal::At{index, signal});
				};
				self[index].changeable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn));
			},
		}
	}
	
	fn reset_view_signals(&self) -> Vec<VecSignal<C::SignalType>> {
		vec![VecSignal::ReplaceAll]
	}
}

impl<T: 'static + Revertable<C> + Send, C: Change> Revertable<VecChange<T, C>> for Vec<T> {
	fn revertable_apply(&mut self, change: VecChange<T, C>, watcher: &mut Watcher<VecSignal<C::SignalType>>) -> VecChange<T, C> {
		use self::VecChange::*;
		match change {
			Set{index, mut item} => {
				std::mem::swap(&mut self[index], &mut item);
				watcher.send_signal(VecSignal::Set{index});
				Set{index, item}
			},
			Insert{index, item} => {
				self.insert(index, item);
				Remove{index}
			},
			Remove{index} => {
				let item = self.remove(index);
				watcher.send_signal(VecSignal::Insert{index});
				Insert{index, item}
			},
			ReplaceAll(mut all) => {
				std::mem::swap(self, &mut all);
				watcher.send_signal(VecSignal::ReplaceAll);
				ReplaceAll(all)
			},
			At{index, change: subchange} => {
				let mut watcher_fn = |signal| {
					watcher.send_signal(VecSignal::At{index, signal});
				};
				let revertchange = self[index].revertable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn));
				At { index, change: revertchange }
			},
		}
	}
}

pub struct VecChangeConstructor<C: Change> {
	index: usize,
	sub: Box<ChangeConstructor<C>>,
}

impl<C: Change> VecChangeConstructor<C> {
	pub fn new(index: usize, sub: Box<ChangeConstructor<C>>) -> VecChangeConstructor<C> {
		VecChangeConstructor { index, sub }
	}
}

impl<T: 'static + Changeable<C>, C: Change> ChangeConstructor<VecChange<T, C>> for VecChangeConstructor<C> {
	fn create(&self, leaf_change: Box<std::any::Any>) -> VecChange<T, C> {
		VecChange::At{
			index: self.index,
			change: self.sub.create(leaf_change)
		}
	}
	
	fn update(&mut self, change: &VecChange<T, C>) -> bool {
		use self::VecChange::*;
		match *change {
			Set{index, ..} if index == self.index => {
				// Set essentially removes the item at the index.
				false
			}
			Insert{index, ..} if index <= self.index => {
				self.index += 1;
				true
			}
			/*VecChange::Move(from, to) if from == self.index => {
				self.index = to;
				true
			}*/
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
	
	fn debug_string(&self) -> String {
		format!("[{}]/{}", self.index, self.sub.debug_string())
	}
}

pub type ValueVecChange<T> = VecChange<T, ValueChange<T>>;
pub type ValueVecSignal = VecSignal<ValueSignal>;

#[cfg(test)]
mod tests {
	use super::*;
	use model::{SubChangeConstructor, LeafChangeConstructor, SpyWatcher};
	
	#[test] fn change_vec() {
		let mut spy = SpyWatcher::new();
		
		let mut v: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
		let revert1 = v.revertable_apply(VecChange::Set::<i32, ValueChange<i32>>{index: 5, item: 7}, &mut spy);
		let revert2 = v.revertable_apply(VecChange::Set::<i32, ValueChange<i32>>{index: 3, item: 8}, &mut spy);
		let revert3 = v.revertable_apply(VecChange::Set::<i32, ValueChange<i32>>{index: 0, item: 9}, &mut spy);
		assert_eq!(v, vec![9, 1, 2, 8, 4, 7]);
		v.revertable_apply(revert3, &mut spy);
		v.revertable_apply(revert2, &mut spy);
		v.revertable_apply(revert1, &mut spy);
		assert_eq!(v, vec![0, 1, 2, 3, 4, 5]);
		v.changeable_apply(VecChange::Set::<i32, ValueChange<i32>>{index: 5, item: 7}, &mut spy);
		v.changeable_apply(VecChange::Set::<i32, ValueChange<i32>>{index: 3, item: 8}, &mut spy);
		v.changeable_apply(VecChange::Set::<i32, ValueChange<i32>>{index: 0, item: 9}, &mut spy);
		assert_eq!(v, vec![9, 1, 2, 8, 4, 7]);
		
		assert_eq!(spy.signals, vec![
			VecSignal::Set{index: 5}, VecSignal::Set{index: 3}, VecSignal::Set{index: 0},
			VecSignal::Set{index: 0}, VecSignal::Set{index: 3}, VecSignal::Set{index: 5},
			VecSignal::Set{index: 5}, VecSignal::Set{index: 3}, VecSignal::Set{index: 0},
		]);
	}
	
	#[test] fn vec_change_constructor() {
		let lcc = LeafChangeConstructor::<ValueChange<u32>>::new();
		let vcc = VecChangeConstructor::<ValueChange<u32>>::new(12, Box::new(lcc));
	}
}
