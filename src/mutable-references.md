# Mutable references

The mutable reference type in the mental model is written `&mut [T .. S]`. It means that you have a
pointer with (temporary) exclusive access to a value of type `T`, and that you promise that the
value will be of type `S` when the borrow ends. The syntax `[T .. S]` represents the type of the
value going from `T` to `S` over the lifetime of the borrow.

Another way to look at this, is that a mutable reference is made of 2 things:
- A pointer to a temporarily owned value of type `T`.
- A function taking the same pointer but with type `S`. This function is automatically called when
  the borrow ends to return the ownership of the value to the lender.

Yet another way to look at this, is that mutable references are like reversed functions:
- Functions `fn(P) -> R` ask for a value of type `P` then give back a value of type `R`.
- Mutable references `&mut [T .. S]` give a value of type `T` then ask it back at type `S`.

## Variance

The mutable reference type `&mut [T .. S]` is co-variant in `T` and contra-variant in `S`. When
enforcing `T` and `S` to be syntactically equal, like this is done with the `&mut T` notation, the
type naturally becomes invariant in `T`. Being able to dissociate `T` and `S` is important to
understand unsafe, and mutable references in the mental model will explicitly dissociate those 2
types by using the `&mut [T .. S]` notation.

## Borrowing

When lending a place of type `T`, the lender may choose the type `S`, that the place should have
when returned by the borrower. For example, if a lender has a place `x` of type `T` and creates
`&mut x` for a borrower, they may ask the borrower to return the place with type `S`, at which point
the lender would get back ownership of `x` but with type `S` instead of `T`.

Re-borrowing is a particular case of borrowing where the initial place is already borrowed. For
example, if the lender (itself a borrower) has a place `x` of type `&mut [T .. S]`, they may create
`&mut *x` of type `&mut [T .. Q]`. The borrower would need to return the place with type `Q`, thus
letting the lender have the place at type `&mut [Q .. S]` instead of `&mut [T .. S]`. In particular,
re-borrowing doesn't affect the final contract with the original lender, who is still expecting the
place to be at type `S`.
