// a simple function to add two numbers
fn addition() {
    let a = 5;
    let b = 10;
    println!("The sum of a and b is {a + b}");
}

//! the main function of the program
fn main() {
    println!("Hello, world!");

    let name = "Jamshaid Nasar";
    println!("Welcome to rust {name}.");

    let mut age = 16;
    println!("I was {age} years old.");

    age = 17;
    println!("I am {age} years old now.");

    println!("\n")
    println!("Addition function:");
    addition();
}
