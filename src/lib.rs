use tupiadic_macros::tupiadic_impl;

/// If you want variadic tuples, [`Variadic`] is the only thing you need to use!
///
/// By adding [`Variadic`] as type constraint for a trait for some impl `T`, you get "free"
/// implementations of that trait for all tuples (and nested tuples) containing items of that trait.
///
/// Variadic essentially determines how to "merge" two slices of a tuple. It is essentially a "fold" / "reduce" operation for tuple items.
/// Functionally, the "fold" happens left-to-right, so the first two items in the tuple will be "folded" first.
///
/// ```
/// use tupiadic::Variadic;
///
/// pub trait Add {
///     fn add(&self) -> usize;
/// }
///
/// impl<T: Variadic<Left: Add, Right: Add>> Add for T {
///     fn add(&self) -> usize {
///         self.left().add() + self.right().add()
///     }
/// }
///
/// pub struct X(usize);
///
/// impl Add for X {
///     fn add(&self) -> usize {
///         self.0
///     }
/// }
///
/// // This works for tuples of 2 or more items
/// assert_eq!(X(1).add(), 1);
/// assert_eq!((X(1), X(2)).add(), 3);
/// assert_eq!(((X(1), X(2)), X(3)).add(), 6);
///
/// // Nested tuples are supported
/// assert_eq!((X(1), (X(2), X(3))).add(), 6);
///
/// // Tuples of multiple types are supported, provided they all impl the same trait
/// pub struct Y(usize);
///
/// impl Add for Y {
///     fn add(&self) -> usize {
///         self.0
///     }
/// }
///
/// assert_eq!((X(1), Y(2)).add(), 3);
/// ```
pub trait Variadic: Sized {
    type Left;
    type Right;
    fn left(&self) -> &Self::Left;
    fn right(&self) -> &Self::Right;
}

/// This is only used internally. You don't need to care about it unless you are curious about implementation details.
///
/// TupleSlice statically represents a "slice" of a tuple.
/// `INDEX` is a zero-indexed offset _from the right_, which defines the "slice".
/// For some tuple `(T1, T2, T3, T4)`:
/// * `TupleSlice<(T1, T2, T3, T4), 0>` represents the `(T1, T2, T3, T4)` "slice"
/// * `TupleSlice<(T1, T2, T3, T4), 1>` represents the `(T1, T2, T3)` "slice"
/// * `TupleSlice<(T1, T2, T3, T4), 2>` represents the `(T1, T2)` "slice"
///
/// The `(T1, T2)` slice is considered the "leaf". This crate does not create "slices" smaller than `(T1, T2)`
#[repr(transparent)]
pub struct TupleSlice<Tuple, const INDEX: usize>(Tuple);

tupiadic_impl!(12);

impl<T1, T2> Variadic for (T1, T2) {
    type Left = T1;
    type Right = T2;

    fn left(&self) -> &Self::Left {
        &self.0
    }

    fn right(&self) -> &Self::Right {
        &self.1
    }
}

#[cfg(test)]
mod tests {
    use crate::Variadic;

    #[test]
    fn add() {
        pub trait Add {
            fn add(&self) -> usize;
        }

        impl<T: Variadic<Left: Add, Right: Add>> Add for T {
            fn add(&self) -> usize {
                self.left().add() + self.right().add()
            }
        }
        pub struct X(usize);

        impl Add for X {
            fn add(&self) -> usize {
                self.0
            }
        }

        pub struct Y(usize);

        impl Add for Y {
            fn add(&self) -> usize {
                self.0
            }
        }

        assert_eq!(X(1).add(), 1);
        assert_eq!((X(1), Y(2)).add(), 3);
        assert_eq!((X(1), (Y(2), X(3))).add(), 6);
        assert_eq!(((X(1), Y(2)), X(3)).add(), 6);
        assert_eq!((X(1), Y(2), X(3)).add(), 6);
        assert_eq!(((X(1), Y(2), X(3)), Y(4)).add(), 10);

        // This test largely exists to ensure multiple impls play nicely with each other
        pub trait Mult {
            fn mult(&self) -> usize;
        }

        impl Mult for X {
            fn mult(&self) -> usize {
                self.0
            }
        }

        impl Mult for Y {
            fn mult(&self) -> usize {
                self.0
            }
        }

        impl<T: Variadic<Left: Mult, Right: Mult>> Mult for T {
            fn mult(&self) -> usize {
                self.left().mult() * self.right().mult()
            }
        }
        assert_eq!((X(2), Y(3)).mult(), 6);
    }
}
