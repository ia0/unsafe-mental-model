# The robust keyword

Robust is the dual of unsafe with respect to variance. It is thus natural to imagine a notation for
it. The main reason this doesn't exist is because it is not recommended to actively document a type
as robust. This should only be done when the robustness of the type is needed for a proof, thus
justifying the burden of making sure the implementation is actually robust.

Let's look at a world where `robust` is the dual keyword of `unsafe`. We'll make the update type
explicit for clarity, but it may be omitted.

```rust
{{#include code.rs:robust-example}}
```

Note that a function may be both robust and unsafe.

```rust
{{#include code.rs:robust-unsafe-example}}
```

The robust keyword would also bring clarity to unsafe traits. When a trait is unsafe only because
some of its non-inherited methods or associated constants are robust, then it actually doesn't need
to be unsafe.

```rust
{{#include code.rs:allocator}}
```

Notice how implementing the trait is safe. Implementing `allocate()` will require some proof of
robustness and thus be "unsafe" to implement. Implementing `deallocate()` on the other hand is
"safe". One can simply leak the pointer. However in practice, one will probably modify the allocator
internal state to be able to reuse that small allocation. This will require some proofs that some
allocator invariant is preserved, which is "unsafe". The next chapter discusses exactly this topic
of invariants.
