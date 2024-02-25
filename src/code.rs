// -*- mode: Rust; fill-column: 80; -*-

// ANCHOR: update-type
/// Creates an anonymous type from a source type.
///
/// The created anonymous type (more precisely its interpretation) is the set of
/// valid values of the source type (its validity invariant) satisfying a
/// predicate.
///
/// The predicate is usually left implicit and defined in some documentation,
/// thus writing `Update<Source>`.
#[repr(transparent)]
pub struct Update<Predicate, Source: ?Sized> {
    /// A proof that the value satisfies the predicate.
    ///
    /// The proof is usually left implicit and defined in some comment, thus
    /// writing `Update { value }`. We may also abusively write `Update(value)`
    /// instead.
    pub proof: Predicate,

    /// The value whose type is being updated.
    ///
    /// It is a valid value of the source type that satisfies the predicate.
    pub value: Source,
}
// ANCHOR_END: update-type

// ANCHOR: proof-type
/// The proof that a predicate holds.
pub type Proof<Predicate> = Update<Predicate, ()>;
// ANCHOR_END: proof-type

// ANCHOR: ptr-read
/// Reads the value from a pointer without moving it.
///
/// # Safety
///
/// The pointer must be valid for reads, aligned, and pointing to a safe value.
pub fn read<T>(ptr: Update<*const T>) -> T;
// ANCHOR_END: ptr-read

// ANCHOR: pin-get-unchecked-mut
/// Gets a mutable reference to the pinned data.
///
/// # Safety
///
/// The data in the result must not be moved.
pub fn get_unchecked_mut<T>(pin: Pin<&mut T>) -> Update<&mut T>
// ANCHOR_END: pin-get-unchecked-mut

// ANCHOR: pin-get-unchecked-mut-split
/// Gets a mutable reference to the pinned data.
///
/// # Safety
///
/// The data in the result at the end of the borrow has not been moved.
pub fn get_unchecked_mut<T>(pin: Pin<&mut T>) -> &mut [T .. Update<T>]
// ANCHOR_END: pin-get-unchecked-mut-split

// ANCHOR: box-into-raw
/// Consumes a box returning its raw pointer.
///
/// # Robustness
///
/// The pointer will be properly aligned and non-null.
pub fn into_raw(b: Box<T>) -> Update<*mut T>
// ANCHOR_END: box-into-raw

// ANCHOR: robust-print
/// Prints a message to the standard output.
///
/// # Robustness
///
/// The message doesn't need to be valid UTF-8.
pub fn robust_print(msg: Update<&str>);
// ANCHOR_END: robust-print

// ANCHOR: box-non-null
/// Consumes a box returning its non-null pointer.
pub fn into_non_null(b: Box<T>) -> NonNull<T> {
    let Update(ptr) = Box::into_raw(b);
    // SAFETY: The pointer is non-null because Box::into_raw() returns a proof
    // that it's non-null and properly aligned.
    NonNull::new_unchecked(Update(ptr.value))
}
// ANCHOR_END: box-non-null

// ANCHOR: non-null
/// A non-null pointer.
#[repr(transparent)]
pub struct NonNull<T: ?Sized> {
    /// The wrapped pointer.
    ///
    /// # Robustness
    ///
    /// The pointer is non-null.
    ptr: Update<*mut T>,
}
// ANCHOR_END: non-null

// ANCHOR: pin
/// A pinned pointer.
#[repr(transparent)]
pub struct Pin<P> {
    /// The wrapped pointer.
    ///
    /// # Safety
    ///
    /// The pointer cannot be moved unless it implements `Unpin`.
    ptr: Update<P>,
}
// ANCHOR_END: pin

// ANCHOR: robust-example
/// Prints a message to the standard output.
///
/// # Robustness
///
/// The message doesn't need to be valid UTF-8.
pub robust fn supersafe_print(msg: &Update<str>);

/// Consumes a box returning its raw pointer.
///
/// # Robustness
///
/// The pointer will be properly aligned and non-null.
pub robust fn into_raw(b: Box<T>) -> Update<*mut T>
// ANCHOR_END: robust-example

// ANCHOR: robust-unsafe-example
/// Returns the n-th valid `char` of a string.
///
/// # Robustness
///
/// The string doesn't need to be valid UTF-8.
///
/// # Safety
///
/// There must be at least `n` valid `char`s in `s`. A `char` is valid in the
/// string if it is the first sequence of bytes that form a UTF-8 `char` since
/// the previous valid `char` in the string (or the beginning of the string).
pub robust unsafe fn chars_nth(s: &Update<str>, n: Update<usize>) -> char;
// ANCHOR_END: robust-unsafe-example

// ANCHOR: allocator
/// Dynamic management of some large owned allocation.
trait Allocator {
    /// Allocates a small allocation from the large allocation.
    ///
    /// # Errors
    ///
    /// Returns an error in the following conditions:
    /// - The layout is not supported by the allocator.
    /// - There is not enough available space in the large allocation.
    ///
    /// # Robustness
    ///
    /// The result points to a small allocation from the large allocation with
    /// the following properties:
    /// - It does not alias with any previously allocated (and not yet
    ///   deallocated) small allocations.
    /// - It is not null (a consequence of coming from the large allocation).
    /// - It satisfies the layout (correct size and alignment).
    ///
    /// Note that the small allocation is uninitialized and thus not valid for
    /// read.
    robust fn allocate(&self, layout: Layout) -> Result<Update<*mut u8>, AllocError>;

    /// Deallocates a small allocation back to the large allocation.
    ///
    /// # Safety
    ///
    /// The following properties must hold:
    /// - The pointer comes from a previous call to `allocate()` with the same
    ///   layout.
    /// - The pointer has not yet been deallocated yet (a consequence of the
    ///   next point).
    /// - The pointer will not be used anymore. The ownership of the small
    ///   allocation it represents is given back to the allocator.
    unsafe fn deallocate(&self, ptr: Update<*mut u8>, layout: Layout);
}
// ANCHOR_END: allocator

// ANCHOR: vec1024-len
/// Vectors of at most 1024 bytes.
pub struct Vec1024 {
    /// Storage of the vector.
    ///
    /// # Robustness
    ///
    /// Points to an allocation of 1024 bytes and is owned.
    ptr: Update<*mut u8>,

    /// Length of the vector.
    ///
    /// # Robustness
    ///
    /// - Always smaller than or equal to 1024.
    /// - The storage prefix of that length is initialized.
    len: Update<usize>,
}

impl Vec1024 {
    pub fn push(&mut self, x: u8) {
        assert!(self.len < 1024);

        // SAFETY: The addition stays within the allocation by the check above.
        // The write is thus also valid.
        unsafe { self.ptr.add(self.len).write(x) };

        // SAFETY: The new length is safe because:
        // - The old length was smaller than 1024 by the check above and only 1
        //   is added, it is thus smaller than or equal to 1024.
        // - The pointed prefix is initialized because it was initialized up to
        //   the last byte and we just wrote the last byte.
        unsafe { self.len += 1 };
    }
}
// ANCHOR_END: vec1024-len

// ANCHOR: vec1024
/// Vectors of at most 1024 bytes.
pub struct Vec1024 {
    ptr: *mut u8,
    len: usize,

    /// Invariant.
    ///
    /// # Robustness
    ///
    /// - The pointer points to an allocation of 1024 bytes and is owned.
    /// - The length is always smaller than or equal to 1024.
    /// - The pointed prefix of size length is initialized.
    inv: Proof,
}

impl Vec1024 {
    pub fn push(&mut self, x: u8) {
        assert!(self.len < 1024);

        // SAFETY: The invariant is trivially preserved (neither the pointer nor
        // the length have been modified). The addition stays within the
        // allocation by the check above. The write is thus also valid.
        unsafe { self.ptr.add(self.len).write(x) };

        // SAFETY: The invariant is preserved:
        // - The length is smaller than or equal to 1024 because it was smaller
        //   than 1024 and only 1 is added.
        // - The pointed prefix is initialized because it was initialized up to
        //   the last byte before and we just wrote the last byte.
        unsafe { self.len += 1 };
    }
}
// ANCHOR_END: vec1024
