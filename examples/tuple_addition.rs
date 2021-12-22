#![recursion_limit = "128"]
// use std::ops::Add;

use tuple_tricks::{NestTuple, PreviousTuple, UnnestTuple};

use make_tuple_traits::mark_tuples;

mark_tuples!(TupleAdditionMarker);

struct DrainTuple<AccumTuple, RemainingTuple> {
    accum: AccumTuple,
    remaining: RemainingTuple,
}

trait DrainableTuplePair {
    type Drained;
    fn drain(self) -> Self::Drained;
}

trait Boo {
    fn boo(self) -> bool;
}

impl<T> Boo for T
where
    T: TupleAdditionMarker,
{
    fn boo(self) -> bool {
        true
    }
}

impl<TupleType, NestedTuple, A, Unnested> DrainableTuplePair for DrainTuple<TupleType, (A,)>
where
    (A,): Sized,
    TupleType: NestTuple<Nested = NestedTuple>,
    (NestedTuple, A): UnnestTuple<Unnested = Unnested>,
{
    type Drained = Unnested;
    fn drain(self) -> Unnested {
        let DrainTuple {
            accum,
            remaining: (a,),
        } = self;
        (accum.nest(), a).unnest()
    }
}

impl<LeftTuple, TailTuple, B, RightTuple, LeftNested, LeftNestedPlusHeadUnnested> DrainableTuplePair
    for DrainTuple<LeftTuple, RightTuple>
where
    LeftTuple: NestTuple<Nested = LeftNested>,
    RightTuple: TupleAdditionMarker + PreviousTuple<Head = B, TailTuple = TailTuple>,
    (LeftNested, B): UnnestTuple<Unnested = LeftNestedPlusHeadUnnested>,
    DrainTuple<LeftNestedPlusHeadUnnested, TailTuple>: DrainableTuplePair,
{
    type Drained =
        <DrainTuple<LeftNestedPlusHeadUnnested, TailTuple> as DrainableTuplePair>::Drained;

    fn drain(self) -> Self::Drained {
        let DrainTuple {
            accum: left_tuple,
            remaining: right_tuple,
        } = self;
        let (right_tail, right_head) = right_tuple.decons();
        let left_nested = left_tuple.nest();
        let accum = (left_nested, right_head).unnest();

        DrainTuple {
            accum,
            remaining: right_tail,
        }
        .drain()
    }
}

trait ReversableTuple {
    type ReverseTuple;
    fn reverse(self) -> Self::ReverseTuple;
}

impl<A> ReversableTuple for (A,) {
    type ReverseTuple = (A,);
    fn reverse(self) -> Self::ReverseTuple {
        self
    }
}

//impl<TupleType, Head, TailTuple> ReversableTuple for TupleType
//where
//    TupleType: TupleAdditionMarker + PreviousTuple<Head = Head, TailTuple = TailTuple>,
//    DrainTuple<(Head,), TailTuple>: DrainableTuplePair,
//{
//    type ReverseTuple = <DrainTuple<(Head,), TailTuple> as DrainableTuplePair>::Drained;
//
//    fn reverse(self) -> Self::ReverseTuple {
//        let (tail, head) = self.decons();
//        DrainTuple {
//            accum: (head,),
//            remaining: tail,
//        }
//        .drain()
//    }
//}
//
trait WrapT {
    type Wrapped;
    fn t(self) -> Self::Wrapped;
}

impl<TupleType: TupleAdditionMarker> WrapT for TupleType {
    type Wrapped = T<TupleType>;
    fn t(self) -> T<TupleType> {
        T(self)
    }
}

//impl<LeftTuple, RightTuple, ReversedRight, DrainedResult> Add<T<RightTuple>> for T<LeftTuple>
//where
//    RightTuple: ReversableTuple<ReverseTuple = ReversedRight>,
//    DrainTuple<LeftTuple, ReversedRight>: DrainableTuplePair<Drained = DrainedResult>,
//{
//    type Output = T<DrainedResult>;
//    fn add(self, T(right_tuple): T<RightTuple>) -> T<DrainedResult> {
//        let T(left_tuple) = self;
//        T(DrainTuple {
//            accum: left_tuple,
//            remaining: right_tuple.reverse(),
//        }
//        .drain())
//    }
//}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct T<TupleType>(TupleType);

impl<TupleType> T<TupleType> {
    fn unwrap(self) -> TupleType {
        let T(tuple) = self;
        tuple
    }
}

fn main() {
    let smol = true;
    let small = 1u8;
    let mid = 1u16;
    let big = 1u32;
    let bigg = 1u64;
    let biggg = 1u128;

    //let left_tuple = (smol, small, mid).t() + (big, bigg, biggg).t();
    let right_tuple = (smol, small, mid, big, bigg, biggg).t();
    let (tail, head) = right_tuple.0.decons();
    let (tail_tail, tail_head) = tail.decons();
    let (tail_tail_tail, tail_tail_head) = tail_tail.decons();
    let tmp = (head, tail_head);
    tmp.boo();

    let right = DrainTuple {
        accum: (tail_tail_tail),
        remaining: (head, tail_head, tail_tail_head),
    };
    (head,).nest();
    right.drain();

    //println!("LEFT TUPLE {:?}", right);
    //assert_eq!(left_tuple, right_tuple);
}
