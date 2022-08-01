use std::io;
use rand:: Rng;

fn main(){

    let truth = rand::thread_rng().gen_range(1..=100);
    println!("{truth}");

    loop {
        println!("Enter your guess:");

        let mut guess = String :: new();
        io:: stdin().read_line(&mut guess).expect("Failed to read guess");
        
        let guess : i32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed {}", guess);

        if guess == truth {
            println!("You guessed correct");
            break;

        }
        else{
            println!("Oho Maa chuda");
            println!("Abe kaha ja raha hai. Chal ek aur baar try kar le.");
        }
    }
    println!("You win!");

}