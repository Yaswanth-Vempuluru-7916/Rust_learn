use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Guess the number!!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is : {}",secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        //The parse method will only work on characters that can logically be converted into numbers and so can easily cause errors.
        //If, for example, the string contained A👍%, there would be no way to convert that to a number
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("You guesses : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Arise up"),
            Ordering::Equal => {
                println!("Congrats mate 🥂");
                break;
            }
            Ordering::Greater => println!("Bow down"),
        }
    }
}
