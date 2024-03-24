# Quick start

This chapter provides an overview of the mental model with practical examples, to let you quickly
decide if the book is for you or not. If you plan to read the book regardless, you can skip this
chapter. It doesn't contain any information that is not already present in the rest of the book.
However, you may come back to it later for a quick refresher.

## Concepts

The mental model relies on the following concepts. You don't need to understand ~~all of them~~ any
of them. They are quite complicated. But knowing they exist is already a first step. You can just
read through and skip anything you don't understand on first read:
- There is a notion of **semantic types**. A semantic type is a set of execution states.
- Semantic types define a **contract** between parts of a program: the **producers** and the
  **consumers**. The contract is that, the producers must produce at most the execution states of
  the type, and the consumers must be able to consume at least the execution states of the type.
- This notion of contract is closely related to the notion of **subtyping**. A semantic type is a
  subtype of another if its set of execution states is included in those of the other type. In
  particular, a contract holds if the producers are a subtype of the consumers (the actual type
  being a witness somewhere in between).
- **Variance** is how functions over semantic types (a function that takes one or more semantic
  types and returns a semantic type) influence subtyping (or equivalently the roles of producers and
  consumers). Co-variance preserves subtyping from the result to the parameter. Contra-variance
  inverses that subtyping (producers of the result become consumers of the parameter, and
  symmetrically consumers of the result become producers of the parameter). In-variance ignores that
  subtyping and requires the parameter to be the same (producers of the result become both producers
  and consumers of the parameter, and similarly for consumers of the result).
- Syntactical types map to semantic types (or functions over semantic types) by their **safety
  invariant** and are thus a subset of semantic types. So we'll just say **types** to mean semantic
  types and explicitly say _syntactical_ otherwise.
- Syntactical types also have a notion of **validity invariant** representing how they get compiled.
  The safety invariant is always a subtype of the validity invariant. Soundness relies on the safety
  invariant while compilation relies on the validity invariant.
- The **update type** `Update<P, T>` updates the safety invariant of a syntactical type `T` with a
  type `P`. The validity invariant is preserved, thus `P` must be a subtype of that validity
  invariant. In practice, the type `P` is almost never syntactical and thus described in
  documentation.
- The update type `Update<P, T>` is **unsafe** if `P \ T` is not empty, and it is **robust** if `T \
  P` is not empty.
- The update type can be **lifted** through syntactical types: `Foo<Update<P, T>>` is the same as
  `Update<Foo<P>, Foo<T>>` by definition. The notions of unsafe types and robust types follow
  variance through lifting.
- **Functions** `fn(P) -> R` are contra-variant in `P` (they consume it) and co-variant in `R` (they
  produce it).
- **Mutable references** have actually 2 types with the same validity invariant. We write them `&mut
  [T .. S]` where `T` is the current type and `S` is the promised type at the end of the borrow.
  They are co-variant in `T` (they produce it) and contra-variant in `S` (they consume it).
- **Unsafe** is when a contract does not hold: a value of type `T` is expected to have type `S` but
  `T` is not a subtype of `S`. In that case, a manual proof that the value is actually of type `S`
  is needed to restore soundness.

## Examples

### slice::get_unchecked()

```rust
/// Safety: P is the set of usize smaller than xs.len()
unsafe fn get_unchecked(xs: &[u8], i: Update<P, usize>) -> &u8;
```

The type `Update<P, usize>` contains all valid integers of type `usize` smaller than `xs.len()`. It
is quite subtle, but notice how the definition of `P` mentions `xs.len()`. To be more precise, `P`
is the set of execution states where `i < xs.len()`. It is attached to `i` because it must hold when
`i` is passed as argument. It's a contract between the caller (producer of `i`) and the callee
(consumer of `i`).

Using arithmetic, we can show that `Update<P, usize>` is at least missing `usize::MAX` from the
safety invariant of `usize`. Because `usize \ P` is not empty (it contains `usize::MAX`), the type
`Update<P, usize>` is a robust type.

Note that `Update<P, usize>` is also a subtype of the validity invariant of `usize` (because it's
the same as its safety invariant) and thus doesn't break compilation. We won't check this in the
future because it's not interesting.

Now we can lift the update type through the function type and get something like this:

```rust
/// Safety: P is the set of usize smaller than xs.len()
get_unchecked: Update<fn(xs: &[u8], i: P) -> &u8, fn(&[u8], usize) -> &u8>;
```

This very long type contains all valid functions of type `fn(&[u8], usize) -> &u8` that only accept
values `i` smaller than `xs.len()`. There are a few things to say:
- It is important to filter from _valid_ functions and not just _safe_ functions. For `usize` it
  didn't make a difference because both the safety and validity invariants are the same type. But
  for functions they are actually different. The validity invariant contains many more functions. It
  contains all the unsafe functions in addition to the safe functions that the safety invariant
  contains.
- While in `Update<P, usize>` we removed some values from the safety invariant, now we are actually
  adding values. This makes the type unsafe, which is why the function is annotated `unsafe fn` and
  documented with a `Safety` section. The fact that the update type changed from robust to unsafe is
  due to variance. It was in a contra-variant position.
- We may wonder what those additional unsafe values are. One of them is actually the implementation
  of `get_unchecked`: a function that would do an out-of-bound access if it were provided a safe
  (more precisely valid) value at `usize` but unsafe at `Update<P, usize>`.

Now that we've looked at the type of `get_unchecked`, let's look at a call site (the function
definition is not interesting).

```rust
// SAFETY: 3 is smaller than 11 which is b"hello world".len()
get_unchecked(b"hello world", unsafe { 3 })
```

By typing we have `3: usize` and we need `3: Update<P, usize>` to call the function. Because `usize`
is not a subtype of `Update<P, usize>` (it's actually the contrary), this cast is unsafe and
requires a manual proof. The manual proof refines the type of `3` from `usize` to `Update<P, usize>`
by looking at the actual execution states and making sure they are all within `Update<P, usize>`. In
this case, the value is always 3 and the value of `b"hello world".len()` is always 11, so the proof
is rather simple.

Note that in practice, update types are usually lifted to function types. We could think that
instead of casting `3` we could cast `get_unchecked` like that:

```rust
// SAFETY: 3 is smaller than 11 which is b"hello world".len()
unsafe { get_unchecked }(b"hello world", 3)
```

But we cannot cast `get_unchecked` back to `fn(&[u8], usize) -> &u8` because `get_unchecked` is
actually not a member of that type. The only solution is to attach the proof to the call-site itself
where the arguments are accessible.

```rust
// SAFETY: 3 is smaller than 11 which is b"hello world".len()
unsafe { get_unchecked(b"hello world", 3) }
```

### Box::into_raw()

```rust
/// Robustness: P is the set of non-null pointers
robust fn into_raw<T>(b: Box<T>) -> Update<P, *mut T>;
```

The type `Update<P, *mut T>` contains all valid (in the sense of the validity invariant, not in the
sense of valid for read or write) pointers of type `*mut T` that are non-null. By definition, this
type is missing `null_mut()` from the safety invariant of `*mut T`. It is thus a robust type.

We can lift the update type through the function type and get:

```rust
/// Robustness: P is the set of non-null pointers
into_raw: for<T> Update<fn(Box<T>) -> P, fn(Box<T>) -> *mut T>;
```

This type is missing some functions that are safe at `for<T> fn(Box<T>) -> *mut T`, in particular
those that return a null pointer. So this type is also robust. This is why the function is annotated
`robust fn` and documented with a `Robustness` section. The fact that lifting the update type
preserved its robustness, is due to variance. It was in a co-variant position.

Let's look at the function definition first (and call-sites later).

```rust
/// Robustness: P is the set of non-null pointers
robust fn into_raw<T>(b: Box<T>) -> Update<P, *mut T> {
    let result: *mut T = [...];
    // SAFETY: The box pointer is non-null by invariant of Box.
    result
}
```

By typing we have `result: *mut T` and we need `result: Update<P, *mut T>` to return from the
function. Because `*mut T` is not a subtype of `Update<P, *mut T>` (it's actually the contrary),
this cast is unsafe and requires a manual proof. The manual proof refines the type of `result` from
`*mut T` to `Update<P, *mut T>` by looking at the actual execution states and making sure they are
all within `Update<P, *mut T>`. In this case, `Box<T>` has an internal invariant that the pointer is
non-null, so the proof simply states this invariant.

Let's now look at a call-site.

```rust
let p: *mut T = Box::into_raw(b);
// SAFETY: The pointer was not modified since it was returned by Box::into_raw
// which ensures it is non-null by robustness. So it is still non-null.
unsafe { NonNull::new_unchecked(p) }
```

First, values may be implicitly cast by subtyping. This is what happens between the result of
`Box::into_raw(b)` of type `Update<P, *mut T>` and the binding type of `p` which is `*mut T`,
because `Update<P, *mut T>` is a subtype of `*mut T`. In particular, `Update<P, *mut T>` is not an
unsafe type, it is only a robust type. And update types that are not unsafe can safely cast to the
type they update (without an unsafe block and a safety comment).

The more interesting fact is how we can transfer the information about execution states after
`Box::into_raw()` returns to the unsafe cast even though we lost the type information. We need to
verify that all execution operations since the last known states produce execution states that match
the contract we need to prove. This is trivially the case here because the result value is not
modified.

In practice, the robustness of `Box::into_raw()` is much more precise than just returning a non-null
pointer.

```rust
/// Robustness: Q is the set of non-aliased, aligned, allocated, and valid pointers
robust fn into_raw<T, P>(b: Box<Update<P, T>>) -> Update<Q, *mut Update<P, T>>;
```

The function is actually polymorphic over the type within the Box, which means it doesn't temper its
content and simply returns the underlying pointer (theorem for free). This correctness property is
not part of the safe type `for<T> fn(Box<T>) -> *mut T`, the function could just return any pointer
(including a null pointer). But updating the parameter and return types we can more precisely
capture the behavior of the function, which matters when proving unsafe code because of the stronger
contract.

The type `Q` is actually a filter over `*mut Update<P, T>` and thus more precisely a function over
types `Q(P)` parametric over `P`. So the result type is actually `Update<Q(P), *mut T>` (the type
being updated doesn't matter, only its validity invariant does).

Functions in the standard library may usually be assumed to have such strong robustness guarantees,
in contrary to other libraries for which correctness may not usually be assumed to hold when proving
unsafe code.

### String::as_mut_vec()

```rust
/// Safety: P is the set of safe Vec<u8> that are UTF-8
unsafe fn as_mut_vec(self: &mut String) -> &mut [Vec<u8> .. Update<P, Vec<u8>>];
```

The type `Update<P, Vec<u8>>` contains all valid values of type `Vec<u8>` that are safe and UTF-8.
It is a robust type because it is missing values that are safe (those that are not UTF-8) and does
not contain unsafe values.

We can lift the update type through the mutable reference and get:

```rust
/// Safety: P is the set of safe Vec<u8> that are UTF-8
unsafe fn as_mut_vec(self: &mut String) -> Update<&mut [Vec<u8> .. P], &mut Vec<u8>>;
```

We get a similar reasoning as with `slice::get_unchecked()` because we have a robust type in a
contra-variant position. The update type contains all valid values of type `&mut Vec<u8>` that
promise UTF-8 at the end of the borrow. So compared to the safe values of type `&mut Vec<u8>`, the
update type contains additional values (thus unsafe at `&mut Vec<u8>`). This makes the update type
unsafe.

We can finally lift the update type through the function type and get:

```rust
/// Safety: P is the set of Vec<u8> that are UTF-8
Update<fn(&mut String) -> &mut [Vec<u8> .. P], fn(&mut String) -> &mut Vec<u8>>
```

This type also has additional values compared to `fn(&mut String) -> &mut Vec<u8>` and is thus
unsafe. This is because the result type of a function is in a co-variant position, and the result
was unsafe. The function type being thus unsafe, it has a `Safety` section and is annotated `unsafe
fn`.

Let's look at a call site.

```rust
/// SAFETY: At the end of the borrow, the message is ASCII, thus UTF-8.
for byte in unsafe { message.as_mut_vec() } {
    *byte &= 0x7f;
}
```

By typing we have `message.as_mut_vec()` of type `&mut [Vec<u8> .. Update<P, Vec<u8>>]` and we need
`&mut Vec<u8>`, i.e. `&mut [Vec<u8> .. Vec<u8>]`. Because this does not hold by subtyping (the
promised type is robust, making the mutable reference unsafe), we need a manual proof. We must
refine the promised type from `Vec<u8>` to `Update<P, Vec<u8>>` by contra-variance of the promised
type, i.e. we must prove that the promised type is UTF-8. Because we convert all bytes to ASCII
before the borrow ends, and don't modify the message further, we can claim that the message at the
end of the borrow is ASCII and thus UTF-8.

In practice, `String::as_mut_vec()` is also robust:

```rust
/// Safety and Robustness: P is the set of safe Vec<u8> that are UTF-8
robust unsafe fn as_mut_vec(self: &mut String) -> &mut Update<P, Vec<u8>>;
```

This function is both robust and unsafe. It is unsafe for the same reason as above (the type is
robust at the end of the borrow) and it is robust because the returned type is robust at the
beginning of the borrow. By being robust, this function type provides more freedom to call-sites.
The following call-site would be unsound with the non-robust function type, but is sound with the
robust function type.

```rust
/// SAFETY: Assuming the message is UTF-8 at the beginning of the borrow, it is still
/// UTF-8 at the end of the borrow.
for bytes in unsafe { message.as_mut_vec() }.chunks_exact_mut(2) {
    // Convert code points encoded with 2 bytes at even offsets to ASCII.
    if bytes[0] & 0xe0 == 0xc0 {
        bytes[0] &= 0x3f;
        bytes[1] &= 0x7f;
    }
}
```

To go even further, because `String::as_mut_vec()` is part of the standard library, it is even more
robust than the previous function type. It is actually polymorphic over the type of `self` (which
contains 2 types because it's a mutable reference):

```rust
/// Safety and Robustness: P and Q are sets of valid values of Vec<u8>
robust unsafe fn as_mut_vec<P, Q>(
    self: Update<&mut [P .. Q], &mut String>
) -> Update<&mut [P .. Q], &mut Vec<u8>>;
```

The most common case is when the parameter is safe. This means both `P` and `Q` are the safety
invariant of `String`, i.e. the set of UTF-8 values. But it is possible to also use unsafe or robust
types (with respect to `String`) for `P` and `Q` independently of each other. An artificial example
could look like this:

```rust
/// Sanitizes a string to make it UTF-8.
///
/// # Robustness
///
/// The message doesn't need to be UTF-8.
robust fn sanitize(message: &mut [Update<Vec<u8>, String> .. String]) {
    // SAFETY: The message is ASCII at the end of the borrow.
    for byte in unsafe { message.as_mut_vec::<Vec<u8>, ASCII>() } {
        *byte &= 0x7f;
    }
    // SAFETY(parameter): The message is ASCII at the beginning of the borrow.
    // SAFETY(result): The message remains ASCII and is thus UTF-8 at the end of the
    // borrow.
    for byte in unsafe { message.as_mut_vec::<ASCII, String>() } {
        *byte ^= 0x07;
    }
}
```

We already discussed the first transformation, so let's focus on the second one. By typing we have
`message` of type `&mut String` and `message.as_mut_vec()` of type `Update<&mut [ASCII .. String],
&mut Vec<u8>>`. We need `message` to have type `Update<&mut [ASCII .. String], &mut String>` and
`message.as_mut_vec()` to have type `&mut Vec<u8>`. Both casts are unsafe because they don't hold by
subtyping. The first one doesn't hold because we have to prove that `message` is initially ASCII,
which we know by preceding code. The second one doesn't hold because we have to prove that
`message.as_mut_vec()` is UTF-8 at the end of the borrow, which we do by using the robustness that
it is initially ASCII and preserving this property to the end of the borrow.
