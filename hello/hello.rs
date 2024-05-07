// This is my first Rust program!

fn main() {
    println!("Hello There! - Obi Wan");
    println!("Don't try it - also Obi Wan");
    println!("Matt is da big gay - Ghandi probably");

    // These are formatted strings in Rust
    
    println!("{}", 800135); // Blank brackets stringify whatever argument you pass :0
    println!("This is argument {0}, and this is argument {1}", "one", "two"); // Pass arguments by indexing                        
    println!("{greeting} {name}", greeting="Hello there, ", name="Obi Wan");

    // Variables

    let a = 1; // Cannot be changed
    let mut b = 4; // Can be changed

    println!("This is before: {}", b);
    b = 7;
    println!("This is after {}", b);

    println!("{}", a);
    println!("{}", a+b);
}
