# Unsafe reviews

Unsafe reviews is the process of reviewing unsafe code for soundness. For those reviews to be
sustainable on code change, it is important to not trigger a review too often while still triggering
each time it is needed. The current approach is to trigger a review when a file containing the
unsafe keyword is modified. This is a good approximation in the sense that if files with the unsafe
keyword are kept small then reviews won't trigger too often. However, it may also miss some reviews
when unsafe code relies on properties outside files with the unsafe keyword (like robust
implementations).

If the robust keyword existed, unsafe reviews could trigger on files containing any of those 2
keywords. By making sure during unsafe review that proofs only rely on documented robust properties
(parameter of unsafe function or result of robust function), this would make sure that unsafe review
will trigger each time it is needed.

To avoid increasing the burden of unsafe reviews, it is important that items are not documented as
robust unless it is known that a proof relies on them. To ensure this, robust items should also
document in their robustness section which crates rely on their robustness. This could alternatively
be tracked in a separate global tool like [`cargo-vet`][cargo-vet].

## Lints

It is almost obligatory to enable [unsafe-op-in-unsafe-fn] which is allowed-by-default up to edition
2021 and warn-by-default starting from edition 2024. Not using this lint will:
- give you a wrong mental model conflating unsafe in types (properties) and unsafe in code (proofs)
  as described in the [RFC][unsafe-op-in-unsafe-fn-rfc],
- conflict with undocumented-unsafe-blocks described below,
- conflict with multiple-unsafe-ops-per-block described below.

The following lints will help unsafe review further:
- [undocumented-unsafe-blocks] is the most important one. Without it, unsafe reviewers have to
  reverse the invariants by reading all the code. Anything non-local should be avoided during
  reviews.
- [multiple-unsafe-ops-per-block] is related but secondary. Without it, the safety comment may
  either (best case scenario) be proving multiple unsafe superpowers being used at the same time
  resulting in possible confusion, or (worst case scenario) forgetting to prove one of the unsafe
  superpowers being used resulting in the same issue as if undocumented-unsafe-blocks was not
  enabled.

Finally, [unused-unsafe] (which is warn-by-default) is the other side of
multiple-unsafe-ops-per-block. Both together ensure that there is a one-to-one correspondence
between the usage of unsafe superpower and the safety comment proving its soundness, thus
simplifying unsafe reviews.

[cargo-vet]: https://github.com/mozilla/cargo-vet
[multiple-unsafe-ops-per-block]: https://rust-lang.github.io/rust-clippy/master/index.html#/multiple_unsafe_ops_per_block
[undocumented-unsafe-blocks]: https://rust-lang.github.io/rust-clippy/master/index.html#/undocumented_unsafe_blocks
[unsafe-op-in-unsafe-fn-rfc]: https://rust-lang.github.io/rfcs/2585-unsafe-block-in-unsafe-fn.html
[unsafe-op-in-unsafe-fn]: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#unsafe-op-in-unsafe-fn
[unused-unsafe]: https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unused-unsafe
