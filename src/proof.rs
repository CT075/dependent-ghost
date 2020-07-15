use std::marker::PhantomData;

pub struct Proof<P> {
    _p: PhantomData<P>,
}

// Attach formulas to values
pub struct SuchThat<A, P> {
    value: A,
    _phantom: PhantomData<P>,
}

impl<A, P> SuchThat<A, P> {
    pub fn into(x: A, _: Proof<P>) -> Self {
        SuchThat {
            value: x,
            _phantom: PhantomData,
        }
    }

    pub fn out(self) -> A {
        self.value
    }

    pub fn out_ref(&self) -> &A {
        &self.value
    }
}

pub fn such_that<A, P>(x: A, p: Proof<P>) -> SuchThat<A, P> {
    SuchThat::into(x, p)
}

// Formulas

pub struct TRUE;
pub struct FALSE;
pub struct And<P, Q> {
    _p: PhantomData<(P, Q)>,
}
pub struct Or<P, Q> {
    _p: PhantomData<(P, Q)>,
}
pub struct Neg<P> {
    _p: PhantomData<P>,
}
pub struct Implies<P, Q> {
    _p: PhantomData<(P, Q)>,
}
pub struct Equiv<P, Q> {
    _p: PhantomData<(P, Q)>,
}

// Need to use this as a function to get around the use of generic `P`
fn qed<P>() -> Proof<P> {
    Proof { _p: PhantomData }
}

// to embed axioms

pub fn axiom<P>() -> Proof<P> {
    qed()
}

// semantics of the combinators...

pub fn and_intro<P, Q>(_: Proof<P>, _: Proof<Q>) -> Proof<And<P, Q>> {
    qed()
}

pub fn and_elim_l<P, Q>(_: Proof<And<P, Q>>) -> Proof<P> {
    qed()
}

pub fn and_elim_r<P, Q>(_: Proof<And<P, Q>>) -> Proof<Q> {
    qed()
}

pub fn or_intro_l<P, Q>(_: Proof<P>) -> Proof<Or<P, Q>> {
    qed()
}

pub fn or_intro_r<P, Q>(_: Proof<Q>) -> Proof<Or<P, Q>> {
    qed()
}

pub fn impl_intro<P, Q, F>(_: F) -> Proof<Implies<P, Q>>
where
    F: Fn(Proof<P>) -> Proof<Q>,
{
    qed()
}

pub fn impl_elim<P, Q>(_: Proof<Implies<P, Q>>, _: Proof<P>) -> Proof<Q> {
    qed()
}

pub fn neg_intro<P, F>(_: F) -> Proof<Neg<P>>
where
    F: Fn(Proof<P>) -> Proof<FALSE>,
{
    qed()
}

pub fn neg_elim<P>(_: Proof<Neg<P>>, _: Proof<P>) -> Proof<FALSE> {
    qed()
}

pub fn false_elim<P>(_: Proof<FALSE>) -> Proof<P> {
    qed()
}

pub fn true_intro() -> Proof<TRUE> {
    qed()
}

pub fn refl<P>() -> Proof<Equiv<P, P>> {
    qed()
}

pub fn equiv_intro<P, Q, F1, F2>(_: F1, _: F2) -> Proof<Equiv<P, Q>>
where
    F1: Fn(Proof<P>) -> Proof<Q>,
    F2: Fn(Proof<Q>) -> Proof<P>,
{
    qed()
}

pub fn equiv_elim<P, Q>(_: Proof<Equiv<P, Q>>, _: Proof<P>) -> Proof<Q> {
    qed()
}

// aliases

pub fn modus_ponens<P, Q>(f: Proof<Implies<P, Q>>, x: Proof<P>) -> Proof<Q> {
    impl_elim(f, x)
}

pub fn absurd<P>(x: Proof<FALSE>) -> Proof<P> {
    false_elim(x)
}

pub fn contradict<P, Q>(x: Proof<P>, f: Proof<Neg<P>>) -> Proof<Q> {
    absurd(neg_elim(f, x))
}
