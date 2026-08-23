fn main() {
    let name = "Asingo paul";
    println!("Welcome to the game of learning Rust {}", name);

    maths();
}

fn maths() {
    let a = 10;
    let b = 100;

    let c = a + b;
    let d = &c;


    println!("The value of C is {}", c);
    println!("The value of d is also {}", d);
}