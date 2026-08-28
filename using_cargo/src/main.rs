
use rand::RngExt; //TngExt is a trait from the rand (crate)


fn main() {
    let number = rand::rng().random_range(1..=100);
    println!("{}", number);
}