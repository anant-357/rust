use std::io;
use rand:: Rng;

fn main(){
    println!("Enter your guess:");

    let truth = rand::thread_rng().gen_range(1..=100);
    println!("{truth}");

    let mut guess = String :: new();
    io:: stdin().read_line(&mut guess).expect("Failed to read guess");
    
    let guess : i32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed {}", guess);

    if guess == truth {
        println!("You guessed correct");

    }
    else{
        println!("Oho Maa chuda");
    }

}