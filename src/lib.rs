use tupiadic_macros::tupiadic_impl;

pub trait VariadicMut<M: VariadicMarker>: Sized {
    fn visit(&mut self, input: &mut M::Input) -> M::Output;
}

pub trait VariadicRef<M: VariadicMarker>: Sized {
    fn visit(&self, input: &mut M::Input) -> M::Output;
}

pub trait VariadicOwned<M: VariadicMarker>: Sized {
    fn visit(self, input: &mut M::Input) -> M::Output;
}

pub trait VariadicMarker {
    type Input;
    type Output;
    fn fold(left: Self::Output, _: Self::Output) -> Self::Output {
        left
    }
}

tupiadic_impl!(12);

#[cfg(test)]
mod tests {
    use crate::{VariadicMarker, VariadicMut};
    #[test]
    fn add() {
        struct X(usize);
        struct Y(usize);

        trait GetUsize {
            fn get_usize(&self) -> usize;
        }

        impl GetUsize for X {
            fn get_usize(&self) -> usize {
                self.0
            }
        }

        impl GetUsize for Y {
            fn get_usize(&self) -> usize {
                self.0
            }
        }

        impl<P: Fn() -> usize> GetUsize for P {
            fn get_usize(&self) -> usize {
                (self)()
            }
        }

        struct AddMarker;

        impl VariadicMarker for AddMarker {
            type Input = ();
            type Output = usize;
            fn fold(left: usize, right: usize) -> usize {
                left + right
            }
        }

        // This trait cannot be implemented outside of this crate due to the orphan rule
        impl<T: GetUsize> VariadicMut<AddMarker> for T {
            fn visit(&mut self, _: &mut ()) -> usize {
                self.get_usize()
            }
        }

        trait Add {
            fn add(&mut self) -> usize;
        }

        impl<T: VariadicMut<AddMarker>> Add for T {
            fn add(&mut self) -> usize {
                self.visit(&mut ())
            }
        }

        assert_eq!(X(1).add(), 1);
        assert_eq!((X(1), X(2)).add(), 3);
        assert_eq!((X(1), (X(2), X(3))).add(), 6);
        assert_eq!((X(1), X(2), Y(3), || 10).add(), 16);
    }
}
