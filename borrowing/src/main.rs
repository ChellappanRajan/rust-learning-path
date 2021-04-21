fn main() {
    //number internally have copy traits implemented so b_number wil have copy of a_number
    // let a_number = 1;
    // let b_number = a_number;

    // let a_string = "A string";
    // let b_string = a_string;
    // here the ownership is moved to b_string so a_string will be have no value it will throw.
    // println!("{}",a_string);
    
    let greeting = String::from("hello");
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    // println!("Greeting: {}", greeting); // We can still use `greeting`;


    fn print_greeting(message: &String) {
        println!("Greeting: {}", message);
    }
      
    
    let greeting = String::from("Hello");
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(greeting_reference); // Since `greeting` didn't move into `print_greeting` we can use it again
  

    borrowing_mutableReference();

}


fn borrowing_mutableReference() {
    let mut value = String::from("hello");
    //Only one mutable (&mut T) reference are allowed
    //so bellow fn ref2 will throw error
    let ref1 = &mut value;
    // let ref2 = &mut value;

    println!("{}", ref1);
}