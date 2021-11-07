use tuple_tricks::PreviousTuple;

trait Marker {}
impl<T> Marker for T where T: PreviousTuple {}

fn main() {
    println!("Hello world!")
}
