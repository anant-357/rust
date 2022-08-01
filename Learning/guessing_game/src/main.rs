use std::io;
use rand:: Rng;

fn main(){

    let truth = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");
        let mut guess = String :: new();
        io:: stdin().read_line(&mut guess).expect("Failed to read guess");

        
        let guess : i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };


        println!("You guessed {}", guess);

        
        if guess == truth {
            println!("You guessed correct");
            break;
        }
        else{
            if guess > truth {
                println!("Too large");
            }
            else{
                println!("Too small");
            }
        }
    }
    println!("You win!");

}