use std::io;
use chrono::{Datelike, Utc};

fn main() {
    loop {
        println!("What is your birth year: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = input.trim().parse().expect("Please type a number!");
        if input == 0 || input > Utc::now().date_naive().year() as u32 || input < 1900 {
            println!("Please enter a valid year");
            continue;
        }
        
        let current_year = Utc::now().date_naive().year();
        let age = current_year as i32 - input as i32;
        println!("Your age is: {}", age);
        break; // Exit the loop after a valid input
    }
}
