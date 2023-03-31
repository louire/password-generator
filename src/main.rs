use std::char::from_u32;
use rand;
use rand::{Rng, thread_rng};

fn main() {
    //Hello there im loui recio and this is a password generator i made in rust :D
    let password_length: u32 = 20; // Set the length of the password
    let mut result: String = String::new(); // Create a new empty string

    for _ in 0..password_length {
        let number: u32 = thread_rng().gen_range(33..126); // Generate a random number between 33 and 126
        let ch : char = from_u32( number).unwrap(); // Convert the number to a character
        result.push(ch); // Add the character to the result string
    }
    


    println!("{}", result);
}
