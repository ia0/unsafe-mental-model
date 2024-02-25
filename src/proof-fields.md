# Proof fields

Fields may use anonymous types, either to be robust and/or unsafe. This usually encodes the
invariant of the struct (or more generally any custom type). Writing to those fields require to
update the proof too. More generally, modifying anything that the predicate relies on, requires to
update the proof.

```rust
{{#include code.rs:vec1024-len}}
```

It is sometimes more convenient to have a single proof field to gather all invariants. One needs to
prove the invariant is preserved each time a field (or something else) referred by the predicate is
modified.

```rust
{{#include code.rs:vec1024}}
```
