use std::cmp::Ordering;
use std::marker::PhantomData;

use dependent_ghost::named::{name, Named};

pub struct SortedBy<Comp, A> {
    value: A,
    _phantom: PhantomData<Comp>,
}

impl<Comp, A> SortedBy<Comp, A> {
    pub fn into(x: A) -> Self {
        SortedBy {
            value: x,
            _phantom: PhantomData,
        }
    }

    pub fn out(self) -> A {
        self.value
    }
}

pub fn sort_by<'a, F, T, C, Comp>(mut v: Vec<T>, cmp: &C) -> SortedBy<Comp, Vec<T>>
where
    F: Fn(&T, &T) -> Ordering,
    C: Named<F, Name = Comp>,
{
    v.sort_by(cmp.out_ref());
    SortedBy::into(v)
}

// I honestly cannot believe that Rust does not have a merge function already.
pub fn merge_by<'a, F, T, C, Comp>(
    xs: SortedBy<Comp, Vec<T>>,
    ys: SortedBy<Comp, Vec<T>>,
    cmp: &C,
) -> SortedBy<Comp, Vec<T>>
where
    F: Fn(&T, &T) -> Ordering,
    C: Named<F, Name = Comp>,
{
    let mut result = Vec::new();
    let mut xs = xs.out().into_iter().peekable();
    let mut ys = ys.out().into_iter().peekable();
    let cmp = cmp.out_ref();

    loop {
        let which = match (xs.peek(), ys.peek()) {
            (Some(x), Some(y)) => Some(cmp(x, y)),
            (Some(_), None) => Some(Ordering::Less),
            (None, Some(_)) => Some(Ordering::Greater),
            (None, None) => None,
        };

        match which {
            None => break,
            Some(Ordering::Less) | Some(Ordering::Equal) => {
                result.push(xs.next().unwrap())
            }
            Some(Ordering::Greater) => result.push(ys.next().unwrap()),
        };
    }

    SortedBy::into(result)
}

#[test]
fn run_sort() {
    let comparator = name(i32::cmp);
    let xs = vec![1, 5, 3];
    let ys = vec![6, 2, 4];
    let xs = sort_by(xs, &comparator);
    let ys = sort_by(ys, &comparator);

    let zs = merge_by(xs, ys, &comparator);

    assert_eq!(zs.out(), vec![1, 2, 3, 4, 5, 6]);
}

/*
fn cmp_backwards(a: &i32, b: &i32) -> Ordering {
    i32::cmp(b, a)
}

pub fn it_doesnt_work() {
    let comp1 = name(i32::cmp);
    let comp2 = name(cmp_backwards);
    let mut xs = vec![1, 3, 5];
    let mut ys = vec![2, 3, 4];

    let xs = sort_by(xs, &comp1);
    let ys = sort_by(ys, &comp2);

    let zs = merge_by(xs, ys, &comp2);
}
*/
