use std::io;

fn main() {
     let error_message = "Failed to read line";

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect(error_message);

    println!("You guessed: {guess}");
}
