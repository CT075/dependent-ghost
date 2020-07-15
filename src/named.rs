use std::marker::PhantomData;

struct _Named<Name, A> {
    value: A,
    _phantom: PhantomData<Name>,
}

struct _Name;

pub trait Named<A> {
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

pub fn name<A>(x: A) -> impl Named<A> {
    _Named::<_Name, A>::into(x)
}
