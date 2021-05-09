/// Basic linear allocator.
pub struct IdAlloc<T> {
	entries: Vec<Option<T>>,
	empty_ids: Vec<usize>,
}

impl<T> IdAlloc<T> {
	pub fn new() -> IdAlloc<T> {
		IdAlloc {
			entries: vec![],
			empty_ids: vec![],
		}
	}
	
	/// Get the entry with the given ID as mut.
	pub fn get_mut(&mut self, id: usize) -> &mut T {
		self.entries[id].as_mut().unwrap()
	}
	
	/// Get the entry with the given ID.
	pub fn get(&self, id: usize) -> &T {
		self.entries[id].as_ref().unwrap()
	}
	
	/// Store the given new_entry with a new ID. Returns (the stored entry, the entry's ID).
	pub fn allocate(&mut self, new_entry: T) -> (&mut T, usize) {
		if let Some(id) = self.empty_ids.pop() {
			let entry = &mut self.entries[id];
			*entry = Some(new_entry);
			(entry.as_mut().unwrap(), id)
		} else {
			let id = self.entries.len();
			self.entries.push(Some(new_entry));
			(self.entries.last_mut().unwrap().as_mut().unwrap(), id)
		}
	}
	
	/// Deallocate the entry with the given ID, freeing up the ID for later allocations.
	pub fn deallocate(&mut self, id: usize) {
		self.entries[id] = None;
		self.empty_ids.push(id);
	}
	
	pub fn apply_to_all(&self, func: &mut FnMut(usize, &T)) {
		for (id, ref opt_entry) in self.entries.iter().enumerate() {
			if let &Some(ref entry) = *opt_entry {
				func(id, entry);
			}
		}
	}
	
	pub fn apply_to_all_mut(&mut self, func: &mut FnMut(usize, &mut T)) {
		for (id, ref mut opt_entry) in self.entries.iter_mut().enumerate() {
			if let &mut Some(ref mut entry) = *opt_entry {
				func(id, entry);
			}
		}
	}
}
