// Learning how to use loops

#[allow(unused_labels)]

fn main(){
    'outer: loop {  // outer and inner are labels that can referance loops
        println!("Welcome to the outer ring of hell!");

        'inner: loop {
            println!("Welcome to the inner ring of hell!");
            break 'outer; // You can specify which loop to break out of or 
        }                 // just use break; to break out of current loop
    }
    println!("You have made it out of hell!");

    // Set value of var with loop
    let mut count = 0;

    let last = loop {
        count += 1;
        if count == 10 {
            break count*100;
        }
    };

    println!("The final count is {}", last);
}
