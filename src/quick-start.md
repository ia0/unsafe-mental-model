# Quick start

This chapter provides an overview of the mental model with practical examples, to let you quickly
decide if the book is for you or not. If you plan to read the book regardless, you can skip this
chapter. It doesn't contain any information that is not already present in the rest of the book.

## Concepts

The mental model relies on the following concepts. You don't need to understand all of them. They
are quite complicated. But knowing they exist is already a first step. You can just read through and
skip any line you don't understand:
- There is a notion of semantic types. A semantic type is a set of execution states.
- Semantic types define a contract between 2 parts of a program: the one "before" and the one
  "after". The contract is that the part "before" produces at most the execution states of the type
  and that the part "after" must consume at least the execution states of the type.
- This notion of contract is closely related to the notion of subtyping. A semantic type is a
  subtype of another if its set of execution states is included in those of the other type.
- Variance is how functions over semantic types (a function that takes one or more semantic types
  and returns a semantic type) influence subtyping. Co-variance preserves subtyping from the
  parameter to the result. Contra-variance inverses that subtyping. When subtyping on parameters
  doesn't result in a subtyping on the results, the function is invariant.
- Syntactical types map to semantic types by their safety invariant.
- Syntactical types also have a notion of validity invariant. The safety invariant is always a
  subtype of the validity invariant. Soundness relies on the safety invariant while compilation
  relies on the validity invariant.
- It is possible to overwrite the semantic type of a syntactical type with `Update<P, T>` where `T`
  is the syntactical type and `P` is the semantic type. To avoid breaking compilation, `P` must be a
  subtype of the validity invariant. The type `P` is usually described in documentation.
- The update type `Update<P, T>` is unsafe if `P \ T` is not empty. And it is robust if `T \ P` is
  not empty.
- The update type can be lifted through syntactical types: `Foo<Update<P, T>>` is the same as
  `Update<Foo<P>, Foo<T>>` where `Foo<P>` is the semantic type defined by `Foo`.
- The notion of unsafe types and robust types follows variance through lifting.
- Functions `fn(P) -> R` are contra-variant in `P` and co-variant in `R`.
- Mutable references have actually 2 semantic types with the same validity invariant. We write them
  `&mut [T .. S]` where `T` is the current type and `S` is the promised type at the end of the
  borrow. It is co-variant in `T` and contra-variant in `S`.
- Unsafe is when a contract does not hold and needs manual fixing.

## Examples

### slice::get_unchecked()

```rust
/// Safety: P is the set of usize smaller than xs.len()
unsafe fn get_unchecked(xs: &[u8], i: Update<P, usize>) -> &u8;
```

The type `Update<P, usize>` contains all valid integers of type `usize` smaller than `xs.len()`.
Because it is at least missing `usize::MAX` from the safety invariant of `usize`, it is a robust
type.

Note that it is also a subtype of the validity invariant of `usize` (because it's the same as its
safety invariant) and thus doesn't break compilation. We won't check this in the future because it's
not interesting.

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
  value at `usize` but unsafe at `Update<P, usize>`.

So to sum up this first example, `get_unchecked` is unsafe because `Update<P, usize>` is robust and
in a contra-variant position.
