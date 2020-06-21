use iterators::Counter;

fn main() {

    let counter = Counter::new();
    for c in counter {
        println!("{}", c);
    }
}
