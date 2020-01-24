use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

	let number = rand::thread_rng().gen_range(1,101); //same as guess, except not mute and using rand instead of string

	let mut guess = String::new(); //let to create variable, mut to make mutable. Stringg::new makes the variable a string

	io::stdin().read_line(&mut guess) //read_line reads the user's input and sets mut guess to the input, & is reference
		.expect("Failed to read line"); //response in case of fail

	println!("You guessed: {}", guess);

	println!("The number was: {}", number);
}
