# Type theory

This chapter is meant for those looking for something more formal. Either how the mental model could
be made more formal, or where does the mental model come from. It is work in progress and can use
your help. Feel free to open issues or pull requests.

## Existing work

The mental model is inspired by the following concepts:
- The [Curry-Howard isomorphism][curry-howard] for internalizing the notions of predicates and
  proofs in the programming language. The decision of doing so being that programmers may be able to
  translate their intuition regarding code and types to proofs and predicates, in particular to
  understand that receiving a proof means you can use it to produce another proof, and that a proof
  implements a predicate.
- [Dependent types][dependent-type] for predicates to depend on the execution environment (the
  memory, variables, etc). In particular, the update type is the sigma type.
- [Denotational semantics][denotational-semantics] for interpreting types as their set of values.

## Breaking the dependency

Using documentation and comments instead of types and code is mostly for practical reasons.

It is impractical to write types like `Proof<"# Safety\n\nThe absolute value of the result is
smaller than or equal to 1000.">` and code like `Update { proof: "// SAFETY: We know that arguments
are small, so some small linear combination is only slightly bigger.", value: () }`.

But moving predicates out of types also breaks the dependency they have on the execution
environment. A consequence of this choice is that it looks like we get some form of type erasure,
but this is only in appearance. Even though the properties are erased from the types, the programmer
must know at runtime the properties of all proofs (both parameters and results). For example, having
a vector of function pointers like `Vec<fn(Proof)>` would need the programmer to know for each
element, what are the properties expected to hold to call the function, because they may differ from
one element to the other.

[curry-howard]: https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence
[denotational-semantics]: https://en.wikipedia.org/wiki/Denotational_semantics
[dependent-type]: https://en.wikipedia.org/wiki/Dependent_type