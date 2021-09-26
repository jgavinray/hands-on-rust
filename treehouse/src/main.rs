#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    let mut allow_them_in = false;
    let visitor_list = ["gavin", "chugz", "ashley", "chloe"];

    println!("Hello, what's your name?");
    let name = what_is_your_name();
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}