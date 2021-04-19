fn main() {
    let mut a_number = 10;
    let a_string = "Hello world";
    let a_boolean = true;

    println!("the number is {}",a_number);
    println!("the boolean is {}", a_boolean);
    println!("the string is {}", a_string);


    a_number = 20;
    println!("the mutated value {}", a_number);

    //Shadowing
    //In rust delcaring new varibale with same name as previous variable name,which 
    //creates a new bindin, In rust this operation is called shadowing.
    let number = 5;
    let number = number + 5;
    let number = 5 * 5;

    println!("the number is {}", number);

}
