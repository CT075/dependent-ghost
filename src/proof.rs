/// Encode proofs of properties
use std::marker::PhantomData;

/// A value of type `Proof<P>` for some type-encoded property `P` is a proof
/// of that property, which can be manipulated to form other proofs. To ensure
/// that such proofs have no runtime impact, they are implemented as zero-size
/// structs utilizing phantom types.
pub struct Proof<P> {
    _p: PhantomData<P>,
}

/// A value of type `SuchThat<A,P>` is a value of type `A` annotated with a
/// proof of the proposition `P`. This is generally used alongside the
/// `Named<A,N>` trait -- a value of type `SuchThat<Named<A, N>, P>` should be
/// read as "a value of type `A` named `N` such that condition `P` holds",
/// where `P` can refer to `N`. For example, we might represent the type of a
/// nonempty vector as `SuchThat<Named<Vec<A>, Xs>, Nonempty<Xs>>`, given that
/// the proposition `Nonempty` is sufficiently defined.
///
/// Note that a value can carry *any* type -- there is no mechanism forcing the
/// proposition to be *about* the wrapped value. The only thing linking the
/// value and the proof is the use of a common name.
pub struct SuchThat<A, P> {
    value: A,
    _phantom: PhantomData<P>,
}

impl<A, P> SuchThat<A, P> {
    fn into(x: A, _: Proof<P>) -> Self {
        SuchThat {
            value: x,
            _phantom: PhantomData,
        }
    }

    /// Extract the value, discarding a proof.
    pub fn out(self) -> A {
        self.value
    }

    /// Take a reference to the value.
    pub fn out_ref(&self) -> &A {
        &self.value
    }
}

/// Annotate a value with a proof.
pub fn such_that<A, P>(x: A, p: Proof<P>) -> SuchThat<A, P> {
    SuchThat::into(x, p)
}

/// # Proposition constructors
///
/// The following logical combinators exist to encode a basic propositional
/// logic (specifically, the rules of intuitionistic natural deduction) into
/// Rust's typesystem. As usual, all of these types are zero-sized and will
/// have no runtime impact.
///
/// With these combinators, users can construct their own proofs of properties
/// that a library author may want to ensure (such as ensuring that a list is
/// non-empty, a key exists in a map, and so on).

/// The trivial true proposition.
pub struct TRUE;
/// The trivial false proposition.
pub struct FALSE;
/// Conjunction.
pub struct And<P, Q> {
    _p: PhantomData<(P, Q)>,
}
/// Disjunction.
pub struct Or<P, Q> {
    _p: PhantomData<(P, Q)>,
}
/// Negation.
pub struct Neg<P> {
    _p: PhantomData<P>,
}
/// Implication.
pub struct Implies<P, Q> {
    _p: PhantomData<(P, Q)>,
}
/// Equivalence. This could be defined in terms of `Implies`, but is provided
/// on its own for convenience.
pub struct Equiv<P, Q> {
    _p: PhantomData<(P, Q)>,
}

// Need to use this as a function to get around the use of generic `P`
fn qed<P>() -> Proof<P> {
    Proof { _p: PhantomData }
}

/// Define an axiom that is trivially true in an underlying theory. This will
/// be most useful in defining domain-specific laws about the behavior of some
/// data structure described by a ghostly type. For example, we might encode
/// the fact that adding a key to a map leaves the other keys untouched as an
/// axiom.
pub fn axiom<P>() -> Proof<P> {
    qed()
}

/// # Proof combinators
///
/// The following functions encode the system of natural deduction as explained
/// above. These should be self-explanatory from the type signatures. In
/// general, use `[connective]_intro` to create something of type
/// `Proof<[connective]>`, and `[connective]_elim` to decompose a connective.

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

/// The following functions are aliases for the above combinators. Use them if
/// you think it makes your logic clearer.

pub fn refl<P>() -> Proof<Equiv<P, P>> {
    qed()
}

pub fn modus_ponens<P, Q>(f: Proof<Implies<P, Q>>, x: Proof<P>) -> Proof<Q> {
    impl_elim(f, x)
}

pub fn absurd<P>(x: Proof<FALSE>) -> Proof<P> {
    false_elim(x)
}

pub fn contradict<P, Q>(x: Proof<P>, f: Proof<Neg<P>>) -> Proof<Q> {
    absurd(neg_elim(f, x))
}
