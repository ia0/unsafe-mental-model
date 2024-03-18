# Navigating Pin

This chapter is not about an unsafe construct, but about the [Pin][pin-type] type of the standard
library. This type can be quite subtle, and this chapter may get some stuff wrong[^issue]. But
because it is strongly related to unsafe, it is a good candidate to demonstrate the mental model on
a practical example.

## What is Pin?

The author believes that Pin is just a set of guidelines to design a pinned typestate. The type
itself doesn't need to be used. One could implement their own typestates PinFooRef, PinFooMut, and
PinFooBox without using Pin. However using the Pin type directly reduces the cognitive load for
users of the API, as they can reuse their knowledge about Pin instead of learning something new.

The type `Pin<T>` is a name for `Update<P, T>` where `P` describes what pinned values of type `T`
are. In particular, `P` is specific to `T`. Different types `T` will have different predicates `P`
to describe their pinned values.

```rust
/// Updates the safety invariant of T to contain pinned values.
pub type Pin<T> = Update<"Pinned values of type T", T>;
```

It is natural to wonder whether `T <: P` must hold[^safe-subtype-pin]. The author believes that it
is not necessary but convenient to have and thus recommended when possible.

An important aspect of Pin is that it is not possible to safely access `T` from `Pin<T>` since they
have different safety invariant. However it is possible to go in both directions with unsafe
functions:

```rust
pub unsafe fn new_unchecked<T>(T) -> Pin<T>;
pub unsafe fn get_unchecked_mut<T>(Pin<&mut T>) -> &mut T;
```

When we look at both of those functions in the mental model (writing `P` for the predicate of a
pinned `T`), we realize that they are just identity functions. They simply wrap or unwrap the
definition of `Pin`.

```rust
pub unsafe fn new_unchecked<T>(Update<P, T>) -> Pin<T>;
pub unsafe fn get_unchecked_mut<T>(Pin<&mut T>) -> Update<P, &mut T>;
```

There are however many subtleties behind those 2 functions. Let's look at them in more details.

### Wrapping Pin

Let's look at mutable references only, because that's the most complex case.

```rust
pub unsafe fn new_unchecked<T>(&mut T) -> Pin<&mut T>;
```

In the mental model, this function is actually polymorphic over both types of the mutable reference.

```rust
pub unsafe fn new_unchecked<T, S>(&mut [T .. S]) -> Pin<&mut [T .. S]>;
```

Because we know both types must have the same validity invariant, let's write `Q` and `R` predicates
over the validity invariant of `T`. The type now becomes:

```rust
(&mut [Update<Q, T> .. Update<R, T>]) -> Pin<&mut [Update<Q, T> .. Update<R, T>]>
//     ^--- A  ---^    ^--- B  ---^                ^---  C ---^    ^---  D ---^
```

We'll use the notation in the comment above (`A`, `B`, `C`, and `D`) to talk about the different
values at the given types. We'll also focus only on 2 predicates: `T` the safety invariant of `T`,
and `P` the safety invariant of `T` in the pinned typestate.

| `Q` | `R` | `A <: Q` | `R <: B` | `Q <: C` | `D <: R` |
|-----|-----|----------|----------|----------|----------|
| `T` | `T` | `T == T` | `T == T` | `T <: P` | `P <: T` |
| `T` | `P` | `T == T` | `P <: T` | `T <: P` | `P == P` |
| `P` | `T` | `T <: P` | `T == T` | `P == P` | `P <: T` |
| `P` | `P` | `T <: P` | `P <: T` | `P == P` | `P == P` |

If `T <: P` then there is nothing to prove for those cells (and that's actually why the function is
robust). However, `P <: T` usually does not hold and needs a proof. For each row, this cell is why
the function is unsafe. We can see that the problem is always the promised type of the mutable
references (`B` and `D`), i.e. what happens at the end of the borrow, because this will inject a
value that is safe at `P` into a value that is safe at `T`, hence the `P <: T` constraint.

There are essentially 2 reasons (assuming `T <: P`) why the function can be unsafe:
- The promised type `R` of the mutable reference is `T`, in which case the result of the function is
  unsafe because its promised type `D` is robust. One must prove that at the end of the borrow, the
  value is actually safe at `T` (i.e. not really pinned).
- The promised type `R` of the mutable reference is `P`, in which case the parameter of the function
  is robust because its promised type `B` is unsafe. One must prove that at the end of the borrow,
  the unsafe values don't cause any problem in the rest of the program.

### Unwrapping Pin

In the mental model, this function is also polymorphic over both types of the mutable reference.
We'll also introduce the predicate parameters `Q` and `R` as above, and name the different values
`A`, `B`, `C`, and `D`.

```rust
(Pin<&mut [Update<Q, T> .. Update<R, T>]>) -> &mut [Update<Q, T> .. Update<R, T>]
//         ^--- A  ---^    ^--- B  ---^             ^---  C ---^    ^---  D ---^
```

We can draw a similar table as above assuming the function is the identity function.

| `Q` | `R` | `A <: Q` | `R <: B` | `Q <: C` | `D <: R` |
|-----|-----|----------|----------|----------|----------|
| `T` | `T` | `P <: T` | `T <: P` | `T == T` | `T == T` |
| `T` | `P` | `P <: T` | `P == P` | `T == T` | `T <: P` |
| `P` | `T` | `P == P` | `T <: P` | `P <: T` | `T == T` |
| `P` | `P` | `P == P` | `P == P` | `P <: T` | `T <: P` |

Same as above, the function is unsafe when the subtyping relation does not hold, usually because of
`P <: T`. Similarly, we can look at the 2 reasons (assuming `T <: P`) why the function can be
unsafe:
- The type `Q` of the mutable reference is `T`, in which case the parameter of the function is
  unsafe because its type `A` is robust. One must prove that the value is actually safe at `T` when
  calling the function.
- The type `Q` of the mutable reference is `P`, in which case the result of the function is unsafe
  because its type `C` is unsafe. One must prove that the unsafe values don't cause any problem.

### Unpin

A type is `Unpin` if `P` is equal to `T`. It essentially has no pinned typestate and a single safety
invariant.

## Example

To be done (see <https://github.com/ia0/unsafe-mental-model/issues/2>).

[^issue]: Please open an issue on Github if you find a bug.
[^safe-subtype-pin]: Definition 3b of
    <https://www.ralfj.de/blog/2018/04/05/a-formal-look-at-pinning.html> requires this subtyping
    relation to hold.

[pin-type]: https://doc.rust-lang.org/std/pin/struct.Pin.html
