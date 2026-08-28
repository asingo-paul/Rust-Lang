// First process of the game is asking the user the input , then process that input, and check that the input is in the expected form

use rand::RngExt;
use std::cmp::Ordering;
use std::io;
//  or we can use but only with the second expressions {use rand;}

// Entry point for the function
fn main() {
    // Print functions always displays on the terminals
    // println!("Guess the number!");
    println!("==== Guess the number to play a game please ====");

    let _secret_number = rand::rng().random_range(1..=50);
    // let _secret_number = rand::random_range(1..50);
    println!("The secret number is {_secret_number}");

    // Allowing multiple guesses with looping
    // we use the loop keyword which creates an infinite loops
    loop {
        println!("Please input your guess.");

        // create a mutable empty string - variable to store the user input
        let mut guess = String::new();

        // recieving the users input
        // now we call in the stdin functionality from the library with the use of io::stdin()
        // Option if e hadn't imported the module with use std::io at the beginning of the program, then we could use the function by writitng the function call as std::io::stdin
        // let _ = io::stdin().read_line(&mut guess);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // read_line then appends the contents the user types into a string without overwriting its contents, so we therefore pass the string as the argument
        // "&" is a reference ------------ which gives you a way to let multiple parts of your code access one piece of data without needing to copy
        // it into the memory multiple times
        // .read_line(&mut guess);// .read_line is a method used to get the users inputs , we are also passing &mut and guess as the arguments

        // convert the guess number from a string type to a number type
        // HANDLING THE INVALID INPUT
        // we will include the result types to check this ...Ok and Err, and the num type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // comparing the secret number and the guessed number,
        // You must add an import from the top to use cmp
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break; // This helps in breaking the loop and exiting the game when the guess matches the _secret_numbers
            }
        }
    }

    // Handling potential errors
    // .expect("Failed to read line");

    // this code can be written is this way too
    // to use without the .expect() method, we can then write this code this way
    // let _ = io::stdin().read_line(&mut guess); -> this makes it asssume the error
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
}
