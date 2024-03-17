# What are types?

This chapter is an informal (and possibly opinionated) overview of type systems. It is necessary to
understand this chapter to understand the mental model, because according to the mental model:

> Unsafe is a way to escape the type system.

## Programming languages

Programming languages usually provide the following components (there may be variations):
- A syntax to write programs, usually some grammar.
- The meaning of programs, usually some operational semantics (how programs execute).

The most common and practical meaning of programs is their execution output (effects and/or values),
which is usually why programs are written and executed. However, for most realistic programming
languages, not all programs have a meaning. In particular, program execution is not always defined
(think undefined behavior). **This is the reason why type systems exist.**

## Type systems

Type systems try to decide whether programs are defined[^goal] in a mechanized way. They tell
whether a program is defined without human interaction, but they may need non-interactive human
assistance in the form of program annotations (see next section).

Type systems come with a proof of _soundness_: if the type system says a program is defined then the
program is defined. However, this classification is an undecidable problem for non-trivial
languages, which is why type systems may classify a program as possibly undefined although it is
actually defined. **This is the reason why unsafe exists.**

Unsafe is a way to make the type system agree that a program is defined. This argumentation is done
informally by a human, and is thus not mechanized. This suddenly introduces the notion of human
errors. The usage of unsafe may thus break the soundness property of the type system. **This is why
soundness must be preserved when using unsafe.**

## Types

Most type systems follow the same techniques. They introduce a notion of types to classify programs.
Types are an additional syntax (with its own grammar) to annotate programs and their parts (like
expressions, variables, statements, functions, etc). The type system provides rules on how those
types combine and interact based on the program structure.

It is natural to interpret types as sets of _values_ (or more generally mathematical objects
describing the possible runtime states at a given point in the execution of the program they are
annotating), called their _type interpretation_ and written `|T|`. For example, the interpretation
`|i32|` of `i32` could be all the 32-bits integers, and `|*const i32|` could be all the pointer-wide
integers with an optional provenance. This interpretation may depend on the execution environment,
like the memory.

Types can also be seen as a form of contract between different parts of the program. This is how,
when all the contracts in a program are satisfied, it is possible to say something about the
program's behavior. In particular, when the output of one part of a program is the input of another
part of the program, the type (or contract) at this junction specifies which values are permitted.
The output program promises to not output values outside the contract, and the input program
promises to accept all values inside the contract.

## Subtyping

Interpreting types as sets of values naturally generates a notion of _subtyping_ through set
inclusion. A type `A` is a subtype of `B`, which we'll write `A <: B`, if `|A|` is included in
`|B|`. For example, all values of type `&'static i32` are also values of type `&'a i32` so `&'static
i32` is a subtype of `&'a i32`.

This notion of subtyping can be used to plug parts of a program together even when they don't have
the same contract. For example, an expression of type `T` can be used where an expression of type
`S` is expected as long as `T` is a subtype of `S`. Because subtyping is a reflexive and transitive
relation, we can always require contracts to match exactly and let the programmer explicitly use
subtyping to change one or the other to make them match.

## Variance

The notions of subtyping and type-level functions naturally generate a notion of _variance_ defining
how `Foo<A>` and `Foo<B>` relate when `A` is a subtype of `B`:
- If `Foo<T>` is co-variant in `T` then `Foo<A>` is a subtype of `Foo<B>`.
- If `Foo<T>` is contra-variant in `T` then `Foo<B>` is a subtype of `Foo<A>`.
- If `Foo<T>` is invariant in `T` then there's no particular relation between `Foo<A>` and `Foo<B>`.

A well-known example of variance is the function type `fn(P) -> R`. It is contra-variant in `P` and
co-variant in `R`. You can call a function with an argument of type `Q` as long as `Q <: P`, and you
can use its result where an expression of type `S` is expected as long as `R <: S`. Equivalently, we
have `fn(P) -> R <: fn(Q) -> S` thus the function also has the type `fn(Q) -> S`.

A less-known (and possibly controversial) example is the mutable reference type `&mut T`. It is
usually assumed to be invariant in `T`. In reality, there are 2 separate occurrences of `T` in `&mut
T`, one of them being co-variant and the other contra-variant. This is discussed in the next
chapter.

[^goal]: Type systems may also be used to decide other properties (like termination) or to generate
    parts of the program. Those alternative roles are not relevant for unsafe and will be ignored.
