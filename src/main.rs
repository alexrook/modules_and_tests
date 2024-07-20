use modules_and_tests::counter::signed::SignedCounter;
use modules_and_tests::pair::{default_pair, Pair};

fn main() {
    println!(
        "In Main default_signed_counter[{:?}]",
        SignedCounter::default_signed_counter()
    );

    let pair: Pair = default_pair();
    println!("In Main default_pair[({},{})]", pair.0, pair.1)
}
