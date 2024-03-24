# The update type

An attempt to add the update type to Rust could be something like this:

```rust
/// Generic wrapper over valid values of type T.
///
/// The predicate P describes which valid values are safe at the wrapped type.
#[lang = "update"]
#[repr(transparent)]
pub struct Update<P, T> {
    predicate: PhantomData<P>,
    value: T,
}

impl<P, T> Update<P, T> {
    /// Wraps a value.
    ///
    /// # Safety
    ///
    /// The predicate P must hold for the parameter.
    ///
    /// # Robustness
    ///
    /// - The parameter doesn't need to be a safe value.
    /// - This is the identity function.
    pub robust unsafe fn wrap(value: T) -> Self {
        Update { predicate: PhantomData<P>, value }
    }

    /// Unwraps a value.
    ///
    /// # Robustness
    ///
    /// - The predicate P holds for the result.
    /// - This is the identity function.
    ///
    /// # Safety
    ///
    /// The result is not necessarily a safe value.
    pub robust unsafe fn unwrap(self) -> T {
        self.value
    }
}
```

It's not clear what restrictions this type needs to have to be safe. It shouldn't implement any
trait that provides safe access to the value (e.g. Deref and DerefMut). Producing (resp. consuming)
an update type should always be unsafe because the predicate may be robust (resp. unsafe).

## Examples

Pin is a specific instance of the update type where `T` implements Deref and DerefMut in a specific
way, such that implementing Deref and DerefMut for Pin is safe.

```rust
/// # Safety
///
/// If `T` implements Deref, it must not break "Pinned T". Idem for DerefMut.
pub struct Pin<T>(Update<"Pinned T", T>);
```
