use std;

pub unsafe fn cast_slice<A, B>(slice_ref: &[A]) -> &[B] {
	use std::slice;

	let raw_len = std::mem::size_of::<A>().wrapping_mul(slice_ref.len());
	let len = raw_len / std::mem::size_of::<B>();
	assert_eq!(raw_len, std::mem::size_of::<B>().wrapping_mul(len));
	slice::from_raw_parts(slice_ref.as_ptr() as *const B, len)
}

pub unsafe fn cast_boxed_slice<A, B>(slice_box: Box<[A]>) -> Box<[B]> {
	// What happens when the capacity is bigger than the Vec's size?
	let raw_len = std::mem::size_of::<A>().wrapping_mul(slice_box.len());
	let slice_ptr = Box::into_raw(slice_box);
	let len = raw_len / std::mem::size_of::<B>();
	assert_eq!(raw_len, std::mem::size_of::<B>().wrapping_mul(len));
	Vec::from_raw_parts((*slice_ptr).as_ptr() as *mut B, len, len).into_boxed_slice()
}
