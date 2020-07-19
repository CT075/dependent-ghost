/// To annotate values with type-level names
use std::marker::PhantomData;

struct _Named<Name, A> {
    value: A,
    _phantom: PhantomData<Name>,
}

struct _Name;

/// A trait describing a value annotated with a type-level `name`, which can
/// then be used to describe certain type-level properties about the value.
/// While this is most useful alongside the combinators provided by the `proof`
/// module, names can also be used as a type-level witness that two values are
/// the same. This is useful for things like sorted lists or nonstandard binary
/// maps, where it is important to ensure that the comparison function used is
/// consistent.
///
/// This trait has been sealed, as library consumers should not implement this
/// trait for themselves. Instead, an opaque
pub trait Named<A>: private::Sealed {
    type Name;
    fn out_ref(&self) -> &A;
    fn out(self) -> A;
}

impl<Name, A> _Named<Name, A> {
    fn into(x: A) -> Self {
        _Named {
            value: x,
            _phantom: PhantomData,
        }
    }
}

impl<Name, A> Named<A> for _Named<Name, A> {
    type Name = Name;
    fn out_ref(&self) -> &A {
        &self.value
    }

    fn out(self) -> A {
        self.value
    }
}

/// Annotate a value with a type-level name. This function returns an
/// existential `impl Named<A>` to ensure that names are fresh per invocation.
/// This prevents consumers from providing their own names, which would violate
/// the invariant that named values represent a property of that value.
///
/// Note that the above does not *quite* work correctly in Rust, as the
/// existential names are not necessarily unique between identical invocations.
pub fn name<A>(x: A) -> impl Named<A> {
    // We do need to specify a type for the `Name` parameter in `_Named`, but
    // it's rendered opaque by the existential quantification.
    _Named::<_Name, A>::into(x)
}

mod private {
    use super::_Named;
    pub trait Sealed {}
    impl<Name, A> Sealed for _Named<Name, A> {}
}
