use std::io;
use std::cmp::Ordering; //enum for comparing values - identifies greater, less or equal value outputs
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //value that returns a new instance of a String type

        io::stdin() //reps standard terminal input
            .read_line(&mut guess)
            .expect("Failed to read line"); //error handling in Result values
        
        let guess: u32 = match guess.trim().parse(){ //handling input parsing to u32, and validations
            Ok(num) => num,
            Err(_) => continue, //validating...executes next iteration; inquires for another guess
        } 

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { //returns a variant of the Ordering enum
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //exit code when the guess is just right
            }
        }
    }
}
