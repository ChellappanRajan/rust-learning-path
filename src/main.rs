use std::io;
use rand::Rng;

//Crate is a collection of rust source code
// We can use Crate to get more functionality the project we are written is building is binary crate, which is an excuteable 
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Please enter the guess");

    let mut guess = String::new();
    // :: this syntex indicates that new is an assosiated function of the string type
    // new function will create new empty string.
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    

    println!("The guessed number is {}", secret_number);


}
