use make_tuple_traits::{make_prev_tuple_types, make_unnest_traits};

pub trait PreviousTuple {
    type TailTuple;
    type Head;
    fn decons(self) -> (Self::TailTuple, Self::Head);
}

pub trait PrevUtil {
    type Head;
    type Tail;

    fn head(self) -> Self::Head;
    fn tail(self) -> Self::Tail;
}

impl<T> PrevUtil for T
where
    T: PreviousTuple,
{
    type Head = <T as PreviousTuple>::Head;
    type Tail = <T as PreviousTuple>::TailTuple;

    fn head(self) -> Self::Head {
        let (_, head) = self.decons();
        head
    }
    fn tail(self) -> Self::Tail {
        let (tail, _) = self.decons();
        tail
    }
}

pub trait NestTuple {
    type Head;
    type Tail;
    type Nested;
    fn nest(self) -> Self::Nested;
}

impl<TupleType, Tail> NestTuple for TupleType
where
    Self: Sized,
    TupleType: PreviousTuple<TailTuple = Tail>,
    Tail: NestTuple,
{
    type Head = <Self as PreviousTuple>::Head;
    type Tail = <Tail as NestTuple>::Nested;
    type Nested = (Self::Tail, Self::Head);

    fn nest(self) -> Self::Nested {
        let (tail, head) = self.decons();
        (tail.nest(), head)
    }
}

impl<A1> NestTuple for (A1,) {
    type Head = ();
    type Tail = Self;
    type Nested = Self;
    fn nest(self) -> Self {
        self
    }
}

pub trait UnnestTuple {
    type Unnested;
    fn unnest(self) -> Self::Unnested;
}

impl<A> UnnestTuple for (A,) {
    type Unnested = Self;
    fn unnest(self) -> Self {
        self
    }
}
make_prev_tuple_types!();

make_unnest_traits!();

#[cfg(test)]
mod test {
    use super::{NestTuple, UnnestTuple};
    #[test]
    fn test_nest() {
        let nested_tuple2 = ((Some(()),), Some(1));
        let b = nested_tuple2.unnest();
        let unnnested_tuple2 = (Some(()), Some(1));

        assert_eq!(b.nest(), nested_tuple2);
        assert_eq!(unnnested_tuple2, b);
    }
}
