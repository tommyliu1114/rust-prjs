use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secrect_number = rand::thread_rng().gen_range(1,101);
    println!("the secret number is : {}",secrect_number);
    println!("please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line.");
    let guess: u32 = guess.trim().parse().expect("please type a number!");
    println!("Your guessed : {}",guess);
    match guess.cmp(&secrect_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
