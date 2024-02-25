# Unsafe traits

A trait can be unsafe for different reasons:
- One of its associated type (including inherited) or the implemented type itself must satisfy a
  predicate.
- One of its method type (including inherited) is robust.
- One of its associated constant type (including inherited) is robust.

Those are documented in the safety section of the trait documentation.

An unsafe trait is translated by adding an associated constant of the proof type, which we'll call
an _associated proof_. The predicate of the proof type is defined in the safety section and
describes why the trait is unsafe.

```rust
unsafe trait Send {}
```

would translate to (preserving any safety documentation):

```rust
trait Send {
    const P: Proof;
}
```
