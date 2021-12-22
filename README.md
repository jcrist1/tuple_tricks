# tuple_tricks
Do tricks with tuples in Rust.

Sometimes you have a tuple of something say `(A, B, C)` and you would like to map all of the types
individually, and have a dedicated tuple, say options or results.  Sometimes you have a bigger
tuple. With this crate you can build induction schema on tuples, so you can map each one separately.
This crate would benefit from the `#![feature(fundamental)]` but until that is done if you want to 
perform tuple tricks you have to mark all of the tuples (up to 32 elements long) with the 
`proc_macro`: `mark_tuples!(YourMarker)`.

# Building an inductive trait on all tuples.
(This is duplicated in the `tuple_tricks` crate).
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
An example for tuples to tuples of options is provided in `tuple_tricks/examples`.

# Example
There is one example of mapping a tuple to a tuple of `Option`. It maps a random
6-tuple of assorted integer/unsized types to `Some`s, using either the inductive
trait implemantation (call with `trait`) or with a manual implementation for 
this specific 6 tuple in this example (call with `manual`). It does this for a 
randomly generate tuple 100000 times
```sh
cargo run --example tuple_induction {ARGUMENT}
```
The trait based map is not entirely zero cost. As it appears to take about 4% more time.

There is another example which adds `std::ops::Add` to (wrapped) tuples, so one
can concatenate tuples with `+`:
```sh
cargo run --example tuple_addition
```

# Similar ideas
I haven't done a very thorough investigation but [frunk](https://github.com/lloydmeta/frunk) 
seems to do some similar stuff.
I couldn't quickly see how I could do some stuff in frunk that I can do here (like
`tuple_addition` example). But that's probably on me. In any case it seems cool.
# Ideas for next steps
It would be nice to get non-consuming methods, so we can keep our old tuples around. I'm open to 
other ideas as issue requests.

Also I was undecided about whether to consider `()` as the empty tuple as this would be more akin 
to induction in peano arithmetic, but decided that implementing a bunch of traits for `()` feels
dangerous. Is there any reason this should be avoided other than my gut feeling? Also I'm assumming
the compiler will optimise out all of the intermediate variables produced by these methods, but you
know what they say happens when you assume...
