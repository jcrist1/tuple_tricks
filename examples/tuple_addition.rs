use std::ops::Add;

use tuple_tricks::{NestTuple, PreviousTuple, UnnestTuple};

use mark_tuple_traits::mark_tuples;

mark_tuples!(TupleAdditionMarker);

trait DrainableTupleSource<TargetTuple> {
    type Drained;
    fn drain(self, target: TargetTuple) -> Self::Drained;
}

impl<TupleType, NestedTuple, A, Unnested> DrainableTupleSource<TupleType> for (A,)
where
    TupleType: NestTuple<Nested = NestedTuple>,
    (NestedTuple, A): UnnestTuple<Unnested = Unnested>,
{
    type Drained = Unnested;
    fn drain(self, tuple: TupleType) -> Unnested {
        (tuple.nest(), self.0).unnest()
    }
}

impl<LeftTuple, TailTuple, Head, RightTuple, LeftNested, LeftNestedPlusHeadUnnested>
    DrainableTupleSource<LeftTuple> for RightTuple
where
    LeftTuple: NestTuple<Nested = LeftNested>,
    RightTuple: TupleAdditionMarker + PreviousTuple<Head = Head, TailTuple = TailTuple>,
    (LeftNested, Head): UnnestTuple<Unnested = LeftNestedPlusHeadUnnested>,
    TailTuple: DrainableTupleSource<LeftNestedPlusHeadUnnested>,
{
    type Drained = <TailTuple as DrainableTupleSource<LeftNestedPlusHeadUnnested>>::Drained;

    fn drain(self, left_tuple: LeftTuple) -> Self::Drained {
        let (right_tail, right_head) = self.decons();
        let left_nested = left_tuple.nest();
        let accum = (left_nested, right_head).unnest();

        right_tail.drain(accum)
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

impl<TupleType, Head, TailTuple> ReversableTuple for TupleType
where
    TupleType: TupleAdditionMarker + PreviousTuple<Head = Head, TailTuple = TailTuple>,
    TailTuple: DrainableTupleSource<(Head,)>,
{
    type ReverseTuple = <TailTuple as DrainableTupleSource<(Head,)>>::Drained;

    fn reverse(self) -> Self::ReverseTuple {
        let (tail, head) = self.decons();
        tail.drain((head,))
    }
}

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

impl<LeftTuple, RightTuple, ReversedRight> Add<T<RightTuple>> for T<LeftTuple>
where
    RightTuple: ReversableTuple<ReverseTuple = ReversedRight>,
    ReversedRight: DrainableTupleSource<LeftTuple>,
{
    type Output = T<<ReversedRight as DrainableTupleSource<LeftTuple>>::Drained>;
    fn add(self, T(right_tuple): T<RightTuple>) -> Self::Output {
        let T(left_tuple) = self;
        T(right_tuple.reverse().drain(left_tuple))
    }
}

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

    let left_tuple = (smol, small, mid).t() + (big, bigg, biggg).t();
    let right_tuple = (smol, small, mid, big, bigg, biggg);

    println!("REVERSED {:?}", right_tuple.reverse());

    assert_eq!(left_tuple.unwrap(), right_tuple);
}
