
/// An example entry. If this is on a stack, you can pop a TypeId, then pop a usize,
/// then use the usize to pop that number of bytes. Now you know the shallowest entry in the stack.
struct Entry {
	data: [u8],
	size: usize,
	type_id: TypeId,
}

struct DeepEnum {
	data: Vec<u8>,
}

// When constructing a DeepEnum, the deepest model struct will want to make a change or a signal, so
// that will be the first to push to the DeepEnum's stack. Then as the parent items "wrap" the
// deeper entries, they will continue to push onto the end of the stack.
// Now, when reading the DeepEnum, you have to start at the end and move backwards to know where the
// entries are.
