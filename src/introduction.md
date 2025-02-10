# Introduction

This book describes a mental model for unsafe in Rust.

## Audience

This book might be for you if:
- You want a coherent and future-proof mental model for unsafe (either to review such code or to
  improve its maintenance).
- You don't mind a bit of type theory (also useful outside unsafe).
- You prefer learning a few general concepts from which many facts can be deduced, rather than
  learning those many facts directly.
- You just care about informal intuition rather than a formal description[^formal].
- You prefer reading and writing Rust than English (Rust is somehow more formal than English).

## Non-goals

This book does not try to:
- Replace existing (and official) documentation regarding unsafe (see [references]).
- Explain what behaviors are undefined or how to write sound unsafe code.
- Introduce new unsafe constructs in Rust (only raise awareness of their absence).
- Formalize unsafe and therefore Rust's type system.

## Limitations

This book has limited usage for a few reasons:
- If you care about correctness as much as soundness, then it is unnecessary and redundant to prove
  soundness, since correctness implies soundness. A similar (and simpler) mental model works for
  correctness, without the concepts of `unsafe` and `robust`.
- The mental model reduces the scope of unsafe to the expression level. This might be too granular
  for practical purposes (which is why it's a mental model and not a formal tool). Most often, being
  modular at function level is enough, in which case the more usual `requires` and `ensures`
  contract style is enough. (Note that requirements may talk about what happens after the function
  returns, which is less surprising in the mental model because it follows a type as contract
  style.)

## Living document

This book is imperfect. Feel free to [open an issue][new-issue] if something is unclear or wrong.
You can also edit the page you are reading with the link in the top right.

## Background

An early attempt of this book was made in this [gist][initial-gist] and discussed in this
[thread][internal-thread].

[^formal]: The appendix attempts to justify this informal intuition with a more formal description
    (if this is really what you want).

[initial-gist]: https://gist.github.com/ia0/820ab50d4c5f0f5e3aeb841cef8e6792
[internal-thread]: https://internals.rust-lang.org/t/simpler-mental-model-for-unsafe/20363
[new-issue]: https://github.com/ia0/unsafe-mental-model/issues/new
[references]: references.md
