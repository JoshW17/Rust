// wtf is rust doing with setting variables equal to if statments and loops

use std::io;
use std::io::Write;

fn main(){
    print!("Enter the name of the chosen one: ");
    io::stdout().flush().unwrap(); // Use to print the above right away and unwrap() implicitly
                                   // handles any errors with flush

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {          // match is just a switch statement
        Ok(n) => {                                     // io::stdin().read_line(&mut input) returns
            println!("{} bytes read", n);              // result enum like a struc that has 
            println!("You entered: {}", input);        // properties Ok(T) and Err(E)
        }                                              // In this case if the statement works it
        Err(error) => println!("error: {error}"),      // returns num bytes read and is it fails
    }                                                  // if retuns the error

    let person = input;
    println!("{}", person);

    let respect =
        if person == "noah\n" {
            println!("How dare you not show the proper respect!!!");
            false
        } else if person == "Noah\n" {
            println!("Ahhh a man of culture and good taste I see.");
            true
        } else {    // I had to add the else for it to compile idk why
            println!("Why would you put the name of anyone else?");
            false
        };
    //   ^ You have to put this here if you are setting a var to a if-else statment
    println!("{}", respect);
}
