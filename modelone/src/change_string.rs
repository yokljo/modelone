use crate::model::{Change, Changeable, Revertable, Watcher};

#[derive(Debug, Clone, PartialEq)]
pub struct StringSignal {
	pub index: usize,
	pub from_len: usize,
	pub to_len: usize,
}

/// A change for String types.
/// Replace the text between [`index`, `index` + `count`) with `new`.
#[derive(Debug, Clone, PartialEq)]
pub struct StringChange {
	pub index: usize,
	pub len: usize,
	pub new: String,
}

impl Change for StringChange {
	type SignalType = StringSignal;
	
	/*fn get_signals(&self) -> Vec<Self::SignalType> {
		vec![StringSignal{index: self.index, from_len: self.len, to_len: self.new.len()}]
	}*/
}

trait StringReplaceRange {
	fn replace_range(&mut self, index: usize, len: usize, new: String) -> String;
}

impl StringReplaceRange for String {
	/// Replaces the text between `index` and `index + len` with `new`, and
	/// returns the string that it replaced.
	fn replace_range(&mut self, index: usize, len: usize, new: String) -> String {
		let old_str: String = self.drain(index .. index + len).collect();
		self.insert_str(index, &new);
		
		old_str
	}
}

impl Revertable<StringChange> for String {
	fn revertable_apply(&mut self, mut change: StringChange, watcher: &mut Watcher<StringSignal>) -> StringChange {
		let old = self.drain(change.index .. change.index + change.len).collect();
		change.len = change.new.len();
		self.insert_str(change.index, &change.new);
		change.new = old;
		
		watcher.send_signal(StringSignal{index: change.index, from_len: change.new.len(), to_len: change.len});
		
		change
	}
}

impl Changeable<StringChange> for String {
	fn changeable_apply(&mut self, change: StringChange, watcher: &mut Watcher<StringSignal>) {
		self.drain(change.index .. change.index + change.len);
		self.insert_str(change.index, &change.new);
		
		watcher.send_signal(StringSignal{index: change.index, from_len: change.len, to_len: change.new.len()});
	}
	
	fn reset_view_signals(&self) -> Vec<StringSignal> {
		vec![StringSignal{index: 0, from_len: usize::max_value(), to_len: self.len()}]
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use model::SpyWatcher;
	
	#[test] fn string_change() {
		let mut spy = SpyWatcher::new();
	
		let mut s = "Hello World".to_string();
		
		{
			let revert = s.revertable_apply(StringChange{index: 6, len: 2, new: "Awesome".into()}, &mut spy);
			assert_eq!(s, "Hello Awesomerld");
			s.revertable_apply(revert, &mut spy);
			assert_eq!(s, "Hello World");
		}
		
		{
			let revert = s.revertable_apply(StringChange{index: 6, len: 4, new: "To".into()}, &mut spy);
			assert_eq!(s, "Hello Tod");
			s.revertable_apply(revert, &mut spy);
			assert_eq!(s, "Hello World");
		}
		
		{
			let revert = s.revertable_apply(StringChange{index: 0, len: 0, new: "".into()}, &mut spy);
			assert_eq!(s, "Hello World");
			s.revertable_apply(revert, &mut spy);
			assert_eq!(s, "Hello World");
		}
		
		s.changeable_apply(StringChange{index: 6, len: 2, new: "Awesome".into()}, &mut spy);
		assert_eq!(s, "Hello Awesomerld");
		s.changeable_apply(StringChange{index: 6, len: 4, new: "To".into()}, &mut spy);
		assert_eq!(s, "Hello Toomerld");
		s.changeable_apply(StringChange{index: 0, len: 0, new: "".into()}, &mut spy);
		assert_eq!(s, "Hello Toomerld");
		
		assert_eq!(spy.signals, vec![
			StringSignal { index: 6, from_len: 2, to_len: 7 },
			StringSignal { index: 6, from_len: 7, to_len: 2 },
			
			StringSignal { index: 6, from_len: 4, to_len: 2 },
			StringSignal { index: 6, from_len: 2, to_len: 4 },
			
			StringSignal { index: 0, from_len: 0, to_len: 0 },
			StringSignal { index: 0, from_len: 0, to_len: 0 },
			
			StringSignal { index: 6, from_len: 2, to_len: 7 },
			StringSignal { index: 6, from_len: 4, to_len: 2 },
			StringSignal { index: 0, from_len: 0, to_len: 0 },
		]);
	}
}
