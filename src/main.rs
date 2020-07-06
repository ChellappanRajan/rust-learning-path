use std::io;
use rand::Rng;
use std::cmp::Ordering;

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
    
    //Her we are shadowin guess string type and converting to inter
    let guess: u32  = guess.trim().parse().expect("Please Enter number"); 


    match secret_number.cmp(&guess) {
        Ordering::Less => println!("Guess is Less"),
        Ordering::Greater => println!("Guess is Greater"),
        Ordering::Equal => println!("Guess is equall")

    }

    println!("The guessed number is {}", secret_number);


}
