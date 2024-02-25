# Unsafe functions

A function (or method) type is unsafe when its interpretation may contain an unsafe value. This is
usually either because it takes a robust type or returns an unsafe type. In those cases, the
appropriate parameter or result type is wrapped under the update type with its predicate defined in
the safety section of the function documentation.

```rust
unsafe fn read<T>(p: *const T) -> T;
unsafe fn get_unchecked_mut<T>(pin: Pin<&mut T>) -> &mut T
```

would translate to (preserving any safety documentation):

```rust
fn read<T>(ptr: Update<*const T>) -> T;
fn get_unchecked_mut<T>(pin: Pin<&mut T>) -> Update<&mut T>
// or with the unfolded mutable reference notation:
fn get_unchecked_mut<T>(pin: Pin<&mut T>) -> &mut [T .. Update<T>]
```

It is also possible that the predicate isn't about a single type, in which case it may be more
elegant to use the proof type:

```rust
unsafe fn unchecked_add(x: i32, y: i32) -> i32;
```

would translate to (preserving any safety documentation):

```rust
fn unchecked_add(x: i32, y: i32, p: Proof) -> i32;
```

This is a function type translation, so it applies to all unsafe function types (i.e. within
function declarations, function definitions, function pointers, and so even when nested within
another type like `Vec<unsafe fn()>`).
