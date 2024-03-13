# Making it explicit

This last chapter gives an explicit syntax to the mental model. This may help understand it better,
but most importantly it facilitates clear discussion about what is actually going on in a program.

Note that we are not actually introducing this syntax to Rust. This is a mental model. We are just
making it explicit in our collective mind.

## The update type

The `Update` type makes anonymous types explicit, including their predicate and proof.

```rust
{{#include code.rs:update-type}}
```

The predicate should be documented in 2 different sections depending on the variance in which the
source type is occurring within the type of the item where the documentation is attached. If the
predicate makes the item type unsafe, then it should be documented in a `# Safety` section. If it
makes the item type robust, then it should be documented in a `# Robustness` section. It is
recommended to avoid making item types robust (and thus avoid robustness sections) until an unsafe
block somewhere depends on this property, to avoid unnecessary work by unsafe reviewers.

The proof should be written in a `// SAFETY:` comment.

## The proof type

A weird but useful special case of the update type is when applied to the unit type. In that case,
the only possible interpretations are the empty set or the singleton set, thus the only information
of this type is whether it is inhabited, which is defined by whether the predicate holds. This type
is really only useful when the predicate depends on the execution environment (the memory, the
stack, the global variables, etc). It can be used to gate parts of the program that must only
execute if a predicate holds, by taking a value of that type.

```rust
{{#include code.rs:proof-type}}
```

## Examples

### Safety documentation

The most common examples of safety documentation are parameters of unsafe functions. The parameters
are robust making the function unsafe due to contra-variance. And they are robust because they have
less safe values, thus providing permissions to use.

```rust
{{#include code.rs:ptr-read}}
```

Less common examples are results of unsafe functions. The results are unsafe making the function
unsafe due to co-variance. And they are unsafe because they have unsafe values, thus enforcing
restrictions to use.

```rust
{{#include code.rs:pin-get-unchecked-mut}}
```

### Robustness documentation

The most common examples of robustness documentation are results of functions. The results are
robust making the function robust due to co-variance. And they are robust because they have less
safe values, thus providing permissions to use.

```rust
{{#include code.rs:box-into-raw}}
```

Less common examples are parameters of functions. The parameters are unsafe making the function
robust due to contra-variance. And they are unsafe because they have unsafe values, thus enforcing
restrictions to use.

```rust
{{#include code.rs:robust-print}}
```

Even though documenting robustness makes the API more precise, it has the downside of increasing the
burden of unsafe reviews. A function for which correctness would otherwise not impact unsafe code,
now has the potential of impacting unsafe code and must be reviewed appropriately. If this potential
is not actually used, then this review is unnecessary.

That said, the standard library can be assumed to be maximally robust (correctness defines
robustness). If the standard library is incorrect, then we have other problems that are as big as
unsoundness of third-party crates (that would be sound if the standard library was correct).

### Safety comments

Safety comments can both produce and use proofs from the update type.

```rust
{{#include code.rs:box-non-null}}
```

### Custom types

Notice how in the previous section, `into_non_null()` replaces the robustness of `into_raw()` with
the safety invariant of `NonNull`, essentially giving a name to its anonymous result type. This is
done by having `NonNull<T>` be a custom type around `Update<*mut T>`. We'll ignore alignment and
variance for simplicity. We'll also ignore the rustc annotations that shrink the validity invariant.

```rust
{{#include code.rs:non-null}}
```

Making a type robust is the most common reason to create custom types, but they can also be used to
make a type unsafe.

```rust
{{#include code.rs:pin}}
```
