# Building an inductive trait on all tuples (up to length 32).
First you will need to provide all tuples with a marker. This is done with 
```rust
mark_tuples!(MyMarker)
```
Then you define a trait that you want to apply to aa tuples of up to length 32:
```rust
trait MyInductiveTrait {
    type AccumulatedTupleType;
    fn fn_to_build_accumulated_type_from_self(self) -> Self::AccumulatedTupleType;
}
```
Then define the trait for the one-tuple (i.e. the start of induction)
```rust
impl<A> MyInductiveTrait for (A,) {
    type AccumulatedTupleType = (MyStruct<A>,);

    fn fn_to_build_accumulated_type_from_self(self) -> Self::AccumulatedTupleType {
        let (a,) = self;
        (MyStruct::new(a),)
    }
}
```
The hardest is the induction step, where you will probably need a good many trait bounds:
```rust
impl<TupleType, Head, PrevTupleType, PrevAccumulatedTuple> MyInductiveTrait for TupleType
where
    TupleType: PreviousTuple<Head = Head, TailTuple = PrevTuple> + MyMarker,
    // You need the `MyMarker` trait because otherwise the compiler complains about potential
    // changes to the PreviousTuple implementation.
    PrevTuple: MyInductiveTrait<AccumulatedTupleType = PrevAccumulatedTuple>,
    PrevAccumulatedTuple: NestTuple,
    (PrevAccumulatedTuple::Nested, MyStruct<Head>): UnnestTuple
{
    ...
}
```
