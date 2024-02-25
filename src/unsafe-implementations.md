# Unsafe implementations

An unsafe implementation is translated by implementing the associated proof.

```rust
// SAFETY: <justification>.
unsafe impl Send for Mutex {}
```

would translate to:

```rust
impl Send for Mutex {
    // SAFETY: <justification>.
    const P: Proof = Update(());
}
```
