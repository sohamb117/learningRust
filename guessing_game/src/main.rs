use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number");
	println!("Bottom bound?");
    let mut bottomBound = String::new();
    io:: stdin().read_line(&mut bottomBound)
        .expect("Failed to read line");
    let bottomBound: u32 = bottomBound.trim().parse()
        .expect("Please type a number!");
	println!("Top bound?");
    let mut topBound = String::new();
    io::stdin().read_line(&mut topBound)
        .expect("Failed to read line");
    let topBound: u32 = topBound.trim().parse()
        .expect("Please type a number!");
    
	let number = rand::thread_rng().gen_range(bottomBound,(topBound+1)); //same as guess, except not mute and using rand instead of string
    
    loop {
        println!("Guess?");
	    let mut guess = String::new(); //let to create variable, mut to make mutable. Stringg::new makes the variable a string
	    io::stdin().read_line(&mut guess) //read_line reads the user's input and sets mut guess to the input, & is reference
		    .expect("Failed to read line"); //response in case of fail
    
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
	    println!("You guessed: {}", guess);
        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
        }
    }
    } 
	println!("The number was: {}", number);
}
