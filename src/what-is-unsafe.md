# What is unsafe?

This chapter provides the mental model of unsafe, relying on the concepts of the previous chapter.

## Anonymous types

Unsafe is a way to locally introduce an _anonymous type_ given its interpretation. In practice, this
is done by reusing the interpretation of an existing type and documenting how that interpretation is
modified. For example, an occurrence of `*const i32` in the program could be documented as being
restricted to values that are valid for read, aligned, and pointing to an initialized `i32`. That
particular occurrence of `*const i32` would actually be an anonymous type created for the occasion
with the documented interpretation, essentially updating the contract at that location.

## Proofs

Recall how using an expression of type `A` where a value of type `B` is expected requires to show
that `A` is a subtype of `B`. When using unsafe, it may happen that this is not the case. This
usually happens when `B` is an anonymous type over `A`, like the `*const i32` example above. In
those cases, one has to manually prove that the possible values of the expression are actually in
`B`. This proof cannot rely on types and must rely on the correctness of the program.

## Validity

Rust has an unusual concept compared to other type systems. It has an additional notion of type
interpretation. Besides the usual interpretation of types which defines the set of _safe values_ (as
defined by the [safety invariant][two-invariants] of the type), there is an interpretation of types
which defines the set of _valid values_ (as defined by the [validity invariant][two-invariants] of
the type). The compiler doesn't know that only safe values are possible, it believes that _unsafe
values_ (values that are valid but not safe) are also possible, and will not optimize if an unsafe
value would invalidate the optimization.

This notion is actually necessary due to type composition: an anonymous type in a contra-variant
position would actually increase the set of possible values. The validity invariant must at least
contain the interpretation of the type where all types in contra-variant positions have been
replaced by the _bottom type_ (the type with an empty interpretation).

## Properties

Unsafe may only update interpretations within the bounds of the validity invariant. In particular,
it may add unsafe values and it may remove safe values, but it may not do anything else. An _unsafe
type_ contains at least one unsafe value and a _robust type_ is missing at least one safe value. By
those definitions, a _safe type_ is neither unsafe nor robust, its interpretation is exactly the
safe values. Only anonymous types may be unsafe and/or robust. In particular, an anonymous type may
be both unsafe and robust.

An unsafe type is a type that has _restrictions to use_, because one needs to take care of those
additional values. A robust type on the contrary is a type that provides _permissions to use_,
because some safe but otherwise problematic values are now absent.

Let's illustrate this with the function type `fn(P) -> R`. By contra-variance, it is unsafe if `P`
is robust. This is the most common case, because `P` has permissions to use that its safe version
would otherwise not have. And by co-variance, it is unsafe if `R` is unsafe. This is less common,
but happens when `R` has restrictions to use, like `Pin::get_unchecked_mut() -> &mut T`.

Similarly for the mutable reference type `&mut T`, which I'll write `&mut [T .. S]` for clarity (see
the last paragraph of the previous chapter if this is unclear). By contra-variance, it is unsafe if
`S` is robust. This is the case with the result type of `String::as_mut_vec() -> &mut Vec<u8>` that
requires permissions to use `S` as UTF-8 (removing the safe values of `Vec<u8>` that are not UTF-8).
Note also how the restriction on the result type of `Pin::get_unchecked_mut() -> &mut T` is actually
a permission on its value at the end of the lifetime, such that after the borrow the lender can
assume the value did not move.

## Custom types

It is interesting to observe that for builtin types, the origin of unsafe seems to always be a
robust type in contra-variant position. We could thus think that it would be enough to only update
types down from the safety invariant instead of the validity invariant. But keeping the option to
add unsafe values to a type is necessary for custom types and reverting the effect of an anonymous
type.

A custom type gives a name to a type expression, its definition. When the custom type is used
instead of its definition, anonymous types can only be introduced directly on the custom type
because the definition is hidden. If one wanted to make the type unsafe by making a contra-variant
type robust within the definition, now one needs to make the custom type unsafe by adding unsafe
values.

Let's use the function type `fn(*const i32)` as initial illustration. If we define a custom type
`Foo` with that function type as definition and we want a specific usage of `Foo` to actually
contain unsafe functions, we can't introduce an anonymous type on `*const i32` because it's hidden
behind the definition of `Foo`. We have to introduce the anonymous type on `Foo`, which is the only
thing visible, and add those unsafe values.

It is also possible for a custom type definition to have an anonymous type. This is for example the
case of `Vec<i32>`. Its simplified definition is an anonymous type on `{ ptr: *mut i32, len: usize,
cap: usize }` removing some safe values. We'll focus on the safe values where not all of the first
`len` elements pointed by `ptr` are initialized. The safety invariant of `Vec<i32>` thus doesn't
contain those (otherwise safe) values, making a pointer to uninitialized data with non-zero length
an unsafe value for that type. One may want to add those unsafe values for some specific occurrences
of `Vec<i32>`, requiring again an anonymous type. This time it's not only because the definition is
hidden, but also because we want to revert the effect of an anonymous type adding back values that
were removed.

Note that, while anonymous types in custom types define a new safety invariant, it is also possible
to define custom types with a different validity invariant using rustc annotations. For example,
`NonZeroI32` has the value zero removed from both the safety invariant and the validity invariant.
This type doesn't have any unsafe value. It is thus not possible to create an unsafe version of that
type. If you would remove the rustc annotations and keep the anonymous type, then zero would not be
safe but still be valid, and thus it would be an unsafe value that can be added to create an unsafe
version of that type.

[two-invariants]: https://www.ralfj.de/blog/2018/08/22/two-kinds-of-invariants.html
