extern crate proc_macro;

use proc_macro::TokenStream;

fn tuple_type_list(letter: &str, n: i8) -> String {
    (1..(n + 1))
        .map(|x| format!("{}{}", letter, x))
        .collect::<Vec<_>>()
        .join(", ")
}

fn make_prev_tuple_type(n: i8) -> String {
    format!(
        "
impl<{upper_tuple_types}> PreviousTuple for ({upper_tuple_types}) {{
    type TailTuple = ({tail_upper_tuple},);

    type Head = {head_upper};

    fn pop(self) -> (Self::TailTuple, Self::Head) {{
        let ({lower_tuple}) = self;
        (({lower_tail_tuple},), {lower_head})
    }}
}}",
        upper_tuple_types = tuple_type_list("A", n),
        tail_upper_tuple = tuple_type_list("A", n - 1),
        head_upper = format!("A{}", n),
        lower_tuple = tuple_type_list("a", n),
        lower_head = format!("a{}", n),
        lower_tail_tuple = tuple_type_list("a", n - 1)
    )
}
fn make_next_tuple_type(n: i8) -> String {
    format!(
        "
impl<{upper_tuple_types}> NextTuple for ({upper_tuple_types}) {{
    type NextTuple<A> = ({upper_tuple_types}, A);

    fn next<A>(self, a: A) -> Self::NextTuple<A> {{
        let ({lower_tuple}) = self;
        ({lower_tuple}, a)
    }}
}}
",
        upper_tuple_types = tuple_type_list("A", n),
        lower_tuple = tuple_type_list("a", n),
    )
}

fn make_nested_tuple(letter: &str, n: i8) -> String {
    (2..(n + 1))
        .map(|x| format!("{}{}", letter, x))
        .fold(format!("({}1,)", letter), |accum, new| {
            format!("({},{})", accum, new)
        })
}

fn make_unnest_trait(n: i8) -> String {
    format!(
        "
impl<{upper_tuple_types}> UnnestTuple for {nested_tuple_types} {{
    type Unnested = ({upper_tuple_types});
    fn unnest(self) -> Self::Unnested {{
        let {lower_nested} = self;
        ({unnested},)
    }}

}}",
        upper_tuple_types = tuple_type_list("A", n),
        nested_tuple_types = make_nested_tuple("A", n),
        lower_nested = make_nested_tuple("a", n),
        unnested = tuple_type_list("a", n),
    )
}

fn mark_tuple(n: i8, marker: &str) -> String {
    format!(
        "
impl<{upper_tuple_types}> {marker} for ({upper_tuple_types},) {{}}
        ",
        upper_tuple_types = tuple_type_list("A", n),
        marker = marker
    )
}

#[proc_macro]
pub fn make_next_tuple_types(_item: TokenStream) -> TokenStream {
    let data = (2..33)
        .map(make_next_tuple_type)
        .collect::<Vec<_>>()
        .join("\n\n");
    data.parse().expect("Couldn't parse the tuple types")
}

#[proc_macro]
pub fn make_prev_tuple_types(_item: TokenStream) -> TokenStream {
    let data = (2..33)
        .map(make_prev_tuple_type)
        .collect::<Vec<_>>()
        .join("\n\n");
    data.parse().expect("Couldn't parse the tuple types")
}

#[proc_macro]
pub fn make_unnest_traits(_item: TokenStream) -> TokenStream {
    (2..33)
        .map(make_unnest_trait)
        .collect::<Vec<_>>()
        .join("\n\n")
        .parse()
        .expect("Couldn't parse unnest traits")
}

#[proc_macro]
pub fn mark_tuples(attr: TokenStream) -> TokenStream {
    let marker = attr.to_string();
    format!(
        "
pub trait {} {{}}
{}
        ",
        &marker,
        (2..33)
            .map(|i| mark_tuple(i, &marker))
            .collect::<Vec<_>>()
            .join("\n\n")
    )
    .parse()
    .expect("Couldn't parse marker traits")
}
