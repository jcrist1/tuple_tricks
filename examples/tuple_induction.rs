use make_tuple_traits::mark_tuples;
use tuple_tricks::NestTuple;
use tuple_tricks::PreviousTuple;
use tuple_tricks::UnnestTuple;

mark_tuples!(AllowedToOptionify);

trait TupleToTupleOfOptions {
    type TupleOfOptionsType;
    fn tuple_of_some(self) -> Self::TupleOfOptionsType;
    fn tuple_of_none(self) -> Self::TupleOfOptionsType;
}

impl<A> TupleToTupleOfOptions for (A,) {
    type TupleOfOptionsType = (Option<A>,);
    fn tuple_of_some(self) -> Self::TupleOfOptionsType {
        let (a,) = self;
        (Some(a),)
    }
    fn tuple_of_none(self) -> Self::TupleOfOptionsType {
        (None,)
    }
}

impl<TupleType, Head, PrevTuple, PrevTupleOption, NestedPrevTupleOption> TupleToTupleOfOptions
    for TupleType
where
    TupleType: PreviousTuple<Head = Head, TailTuple = PrevTuple> + AllowedToOptionify,
    PrevTuple: TupleToTupleOfOptions<TupleOfOptionsType = PrevTupleOption>,
    PrevTupleOption: NestTuple<Nested = NestedPrevTupleOption>,
    (NestedPrevTupleOption, Option<Head>): UnnestTuple,
{
    type TupleOfOptionsType = <(NestedPrevTupleOption, Option<Head>) as UnnestTuple>::Unnested;

    fn tuple_of_some(self) -> Self::TupleOfOptionsType {
        let (prev, head) = self.pop();
        (prev.tuple_of_some().nest(), Some(head)).unnest()
    }

    fn tuple_of_none(self) -> Self::TupleOfOptionsType {
        let (prev, _) = self.pop();
        (prev.tuple_of_none().nest(), None).unnest()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct SomeStruct;
fn main() {
    let tuple = (55i32, String::from("hello"), SomeStruct);
    let cloned_tuple = tuple.clone();
    let some_tuple = tuple.tuple_of_some();
    assert_eq!(
        some_tuple,
        (Some(55), Some(String::from("hello")), Some(SomeStruct))
    );

    assert_eq!(cloned_tuple.tuple_of_none(), (None, None, None));
    println!("All checks passed!");
}
