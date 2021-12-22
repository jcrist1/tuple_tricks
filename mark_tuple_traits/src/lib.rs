extern crate proc_macro;

use proc_macro::TokenStream;

fn tuple_type_list(letter: &str, n: i8) -> String {
    (1..(n + 1))
        .map(|x| format!("{}{}", letter, x))
        .collect::<Vec<_>>()
        .join(", ")
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
