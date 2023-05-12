use std::io::stdin;
use crate::model::{Combination, evaluate};

mod model;

fn main() {
    let combination = Combination::random();
    println!("I have a combination, start guessing");
    println!("{:?}", combination);
    println!("{}", combination);

    let mut line = String::new();
    loop {
        stdin().read_line(&mut line);
        if let Some(guess) = Combination::parse(&line) {
            let response = evaluate(&combination, &guess);
            if response.num_position == 4 {
                println!("you did it");
                break;
            }
            println!("{:?}", response);
        }
        else {
            println!("enter four characters");
        }
    }


}
