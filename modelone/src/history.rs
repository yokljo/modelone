use crate::model::{Change, Changeable, Revertable, Watcher, SubWatcher, apply_pipe_to_mut_ref};

use std::fmt;

#[derive(Clone, PartialEq)]
struct ChangeSet<C> {
	name: String,
	changes: Vec<C>,
}

impl<C> ChangeSet<C> {
	fn new(name: String) -> ChangeSet<C> {
		ChangeSet {
			name,
			changes: vec![],
		}
	}
}

/// Allows undo and redo within a [`Changeable`](trait.Changeable.html) data model. 
#[derive(Clone, PartialEq)]
pub struct History<T: Revertable<C>, C: Change> {
	pub model: T,
	undo_stack: Vec<ChangeSet<C>>,
	redo_stack: Vec<ChangeSet<C>>,
}

impl<T: Revertable<C> + fmt::Debug, C: Change> fmt::Debug for History<T, C> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "History({}<>{}): ", self.undo_stack.len(), self.redo_stack.len())?;
		self.model.fmt(f)
	}
}

impl<T: Revertable<C>, C: Change> History<T, C> {
	pub fn new(model: T) -> History<T, C> {
		History {
			model,
			undo_stack: vec![],
			redo_stack: vec![],
		}
	}
	
	pub fn can_undo(&self) -> bool {
		!self.undo_stack.is_empty()
	}
	
	pub fn can_redo(&self) -> bool {
		!self.redo_stack.is_empty()
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum HistorySignal<ST> {
	/// This signal is for any change that touches the History's model,
	/// including Push/Undo/Redo, except for Reset, which must be handled
	/// separately.
	Change(ST),
	Reset,
	/// Signal for when the current transaction changes.
	CurrentTransaction,
	Push,
	Undo,
	Redo,
	ClearHistory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HistoryChange<T: Revertable<C>, C: Change> {
	Reset(Box<T>),
	NewTransaction(String),
	Push(C),
	Undo,
	Redo,
	ClearHistory,
	//Pop,
}

impl<T: 'static + Revertable<C> + Send, C: Change> Change for HistoryChange<T, C> {
	type SignalType = HistorySignal<C::SignalType>;
	
	/*fn get_signals(&self) -> Vec<Self::SignalType> {
		match *self {
			HistoryChange::Reset(..) => {
				vec![HistorySignal::Reset]
			},
			HistoryChange::NewTransaction(ref name) => {
				vec![HistorySignal::CurrentTransaction]
			},
			HistoryChange::Push(ref change) => {
				change.get_signals().into_iter().map(
					|signal| HistorySignal::Change(signal)
				).chain(Some(HistorySignal::Push).into_iter()).collect()
			},
			HistoryChange::Undo => {
				vec![HistorySignal::Undo]
			},
			HistoryChange::Redo => {
				vec![HistorySignal::Redo]
			},
			HistoryChange::ClearHistory => {
				vec![HistorySignal::ClearHistory]
			},
		}
	}*/
}

impl<T: 'static + Revertable<C> + Send, C: Change> Changeable<HistoryChange<T, C>> for History<T, C> {
	fn changeable_apply(&mut self, change: HistoryChange<T, C>, watcher: &mut Watcher<HistorySignal<C::SignalType>>) {
		use self::HistoryChange::*;
		match change {
			Reset(newmodel) => {
				self.model = *newmodel;
				self.undo_stack.clear();
				self.redo_stack.clear();
				watcher.send_signal(HistorySignal::Reset);
			},
			NewTransaction(name) => {
				self.undo_stack.push(ChangeSet::new(name));
				self.redo_stack.clear();
				watcher.send_signal(HistorySignal::CurrentTransaction);
			},
			Push(subchange) => {
				let revertchange = {
					let mut watcher_fn = |signal| {
						watcher.send_signal(HistorySignal::Change(signal));
					};
					//self.change_queue.push(subchange.clone());
					self.model.revertable_apply(subchange, &mut SubWatcher::new(&mut watcher_fn))
				};
				
				self.redo_stack.clear();
				
				if self.undo_stack.is_empty() {
					// There were no transactions to add to, so just add to a
					// new, nameless one
					let mut first_changeset = ChangeSet::new("".into());
					first_changeset.changes.push(revertchange);
					self.undo_stack.push(first_changeset);
				} else {
					if let Some(ref mut current) = self.undo_stack.last_mut() {
						current.changes.push(revertchange);
					} else {
						panic!("Should not be reachable");
					}
				}
				watcher.send_signal(HistorySignal::Push);
			},
			Undo => {
				if let Some(mut revertchangeset) = self.undo_stack.pop() {
					{
						let mut watcher_fn = |signal| {
							watcher.send_signal(HistorySignal::Change(signal));
						};
						
						for revertchange in &mut revertchangeset.changes.iter_mut().rev() {
							//self.change_queue.push(revertchange.clone());
							apply_pipe_to_mut_ref(|change| self.model.revertable_apply(change, &mut SubWatcher::new(&mut watcher_fn)), revertchange);
						}
					}
					
					self.redo_stack.push(revertchangeset);
					watcher.send_signal(HistorySignal::Undo);
					watcher.send_signal(HistorySignal::CurrentTransaction);
				}
			},
			Redo => {
				if let Some(mut reapplychangeset) = self.redo_stack.pop() {
					{
						let mut watcher_fn = |signal| {
							watcher.send_signal(HistorySignal::Change(signal));
						};
						
						for reapplychange in &mut reapplychangeset.changes.iter_mut() {
							//self.change_queue.push(reapplychange.clone());
							apply_pipe_to_mut_ref(|change| self.model.revertable_apply(change, &mut SubWatcher::new(&mut watcher_fn)), reapplychange);
						}
					}
					
					self.undo_stack.push(reapplychangeset);
					watcher.send_signal(HistorySignal::Redo);
					watcher.send_signal(HistorySignal::CurrentTransaction);
				}
			},
			ClearHistory => {
				self.undo_stack.clear();
				self.redo_stack.clear();
				watcher.send_signal(HistorySignal::ClearHistory);
				watcher.send_signal(HistorySignal::CurrentTransaction);
			},
		}
	}
	
	fn reset_view_signals(&self) -> Vec<HistorySignal<C::SignalType>> {
		vec![HistorySignal::Reset, HistorySignal::CurrentTransaction]
	}
}

/*#[derive(Clone, Debug)]
pub struct Hist<C> {
	undo_stack: Vec<ChangeSet<C>>,
	redo_stack: Vec<ChangeSet<C>>,
}

impl<C> Hist<C> {
	fn push(&mut self, revertchange: C) {
		self.redo_stack.clear();
		
		if self.undo_stack.is_empty() {
			// There were no transactions to add to, so just add to a
			// new, nameless one
			let mut first_changeset = ChangeSet::new("".into());
			first_changeset.changes.push(revertchange);
			self.undo_stack.push(first_changeset);
		} else {
			if let Some(ref mut current) = self.undo_stack.last_mut() {
				current.changes.push(revertchange);
			} else {
				panic!("Should not be reachable");
			}
		}
	}

	fn undo<T: Revertable<C>>(&mut self, cxt: &mut ApplyContext<T, C>) {
		if let Some(mut revertchangeset) = self.undo_stack.pop() {
			for revertchange in &mut revertchangeset.changes.iter_mut().rev() {
				cxt.apply(revertchange);
			}
			
			self.redo_stack.push(revertchangeset);
		}
	}
	
	fn redo<T: Revertable<C>>(&mut self, cxt: &mut ApplyContext<T, C>) {
		if let Some(mut reapplychangeset) = self.redo_stack.pop() {
			for reapplychange in &mut reapplychangeset.changes.iter_mut().rev() {
				cxt.apply(reapplychange);
			}
			
			self.undo_stack.push(reapplychangeset);
		}
	}
}

#[derive(Clone, Debug)]
pub struct ValWithHist<T: Revertable<C>, C> {
	val: T,
	hist: Hist<C>,
}

impl<T: Revertable<C>, C> ValWithHist<T, C> {
	fn get_hist(&mut self) -> Hist<C> {
		self.hist
	}
}

impl<T: Revertable<C>, C> Revertable<C> for ValWithHist<T, C> {
	fn revertable_apply(&mut self, change: C) -> C {
		let revertchange = self.val.revertable_apply(change);
		self.hist.push(revertchange.clone());
		revertchange
	}
}*/
