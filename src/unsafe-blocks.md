# Unsafe blocks

Rust provides the following [unsafe superpowers][unsafe-superpowers]:
- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

Most of them require an unsafe block. When translating an unsafe block to the mental model, the use
of the unsafe superpowers are made explict, preserving the safety comment if any.

## Dereference a raw pointer

Dereferencing a raw pointer can be seen as an operation that takes a value of some robust pointer
type and returns a place. The exact predicate of that robust type is not part of the mental model,
only making the operation take `Update<*const T>` or `Update<*mut T>` is.

```rust
// SAFETY: The pointer is valid, aligned, and points to a safe value.
unsafe { *ptr }
```

would translate to:

```rust
// SAFETY: The pointer is valid, aligned, and points to a safe value.
*Update(ptr)
```

or more pedantically:

```rust
let proof = "The pointer is valid, aligned, and points to a safe value.";
*Update { proof, value: ptr }
```

## Call an unsafe function or method

Calling an unsafe function or method is the same as calling a regular function or method, except
that the parameters using the update type (here `Update<*const T>`) must be adapted.

```rust
// SAFETY: <justification for add>. <justification for read>.
unsafe { ptr.add(start).read() }
```

would translate to:

```rust
// SAFETY: <justification for add>. <justification for read>.
Update(Update(ptr).add(start)).read()
```

Note how the location of the usage of both unsafe superpowers had to be made explicit. In
particular, the pedantic version could use 2 distinct proofs:

```rust
let proof = "justification for add";
let ptr = Update { proof, value: ptr }.add(start);
let proof = "justification for read";
Update { proof, value: ptr }.read()
```

## Access or modify a mutable static variable

Accessing or modifying a mutable static variable can be seen as an operation that takes a mutable
static variable with a proof and returns a place. The proof states that the mutable variable is not
aliased while the place is alive.

```rust
static mut COUNTER: u32 = 0;
// SAFETY: <justification>.
unsafe { COUNTER += 1 };
```

would translate to:

```rust
static mut COUNTER: u32 = 0;
// SAFETY: <justification>.
access_static_mut!(COUNTER, Update(())) += 1;
```

We have to use a macro because we need to pass the mutable static as a name without accessing it.

## Implement an unsafe trait

This doesn't require an unsafe block and will be treated in its own chapter.

## Access fields of unions

Accessing a field of a union can be seen as an operation that takes a place to the union, the field
to access, and a proof, and returns the place of the field. The proof states that accessing that
field of that union for that lifetime is valid.

```rust
// SAFETY: <justification>.
unsafe { u.f1 }
```

would translate to:

```rust
// SAFETY: <justification>.
access_union_field!(u, f1, Update(()))
```

Similarly for mutable static variables, we also need a macro because we need to pass the union as a
place and its field as a name.

[unsafe-superpowers]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#unsafe-superpowers
