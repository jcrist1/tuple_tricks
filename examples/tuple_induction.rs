use mark_tuple_traits::mark_tuples;
use rand_distr::Distribution;
use std::env;
use tuple_tricks::NestTuple;
use tuple_tricks::PreviousTuple;
use tuple_tricks::UnnestTuple;

mark_tuples!(AllowedToOptionify);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct StructA(usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct StructB(isize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct StructC(i8);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct StructD(u8);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct StructE(i16);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct StructF(u16);

#[allow(clippy::many_single_char_names)]
#[allow(clippy::type_complexity)]
fn manual_option(
    (a, b, c, d, e, f): (StructA, StructB, StructC, StructD, StructE, StructF),
) -> (
    Option<StructA>,
    Option<StructB>,
    Option<StructC>,
    Option<StructD>,
    Option<StructE>,
    Option<StructF>,
) {
    (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f))
}

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
        let (prev, head) = self.decons();
        (prev.tuple_of_some().nest(), Some(head)).unnest()
    }

    fn tuple_of_none(self) -> Self::TupleOfOptionsType {
        let (prev, _) = self.decons();
        (prev.tuple_of_none().nest(), None).unnest()
    }
}

#[allow(clippy::many_single_char_names)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();
    let dist = rand_distr::Uniform::new(0, 128);
    let start = std::time::Instant::now();
    for _ in 0..100000 {
        let a = StructA(dist.sample(&mut rng) as usize);
        let b = StructB(dist.sample(&mut rng) as isize);
        let c = StructC(dist.sample(&mut rng) as i8);
        let d = StructD(dist.sample(&mut rng) as u8);
        let e = StructE(dist.sample(&mut rng) as i16);
        let f = StructF(dist.sample(&mut rng) as u16);
        let opt = (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f));
        let orig = (a, b, c, d, e, f);

        let method = args
            .get(1)
            .expect("Must call with one argument.  Either \"trait\" or \"manual\"");
        if method == "trait" {
            assert_eq!(orig.tuple_of_some(), opt);
        } else if method == "manual" {
            assert_eq!(manual_option(orig), opt);
        } else {
            panic!(
                "Unexpected argument received. Use either \"trait\" or \"manual\".  Received: {}",
                method
            )
        }
    }
    let end = std::time::Instant::now();
    let dt = end - start;
    println!("Test took {:?} units of time", dt);
}
