// me trying to learn macros and see why they aren't just functions with extra steps
use rand::Rng;

macro_rules! obi_wan_quotes {
    () => { // () means that the macro takes in no input args
        let quotes: [&str; 5] = ["Hello There!", "Another happy landing.", "Use the force Luke.", "I have the high ground.", "Only a sith deals in absolutes."];
        let rnum = rand::thread_rng().gen_range(0..5); // This gens a rand num from [0..4]???
        println!("{}", quotes[rnum]);
    };
}

fn main() {
    obi_wan_quotes!();
}

