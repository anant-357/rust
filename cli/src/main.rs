
use std::env;
use std::process;
use cli :: Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
<<<<<<< HEAD
        eprintln!("Problem encountered: {}", err);
=======
        println!("Problem encountered: {}", err);
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("in file: {}", config.filename);

    if let Err(e) = cli::run(config){
<<<<<<< HEAD
        eprintln!("Application Error: {}", e);
=======
        println!("Application Error: {}", e);
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
        process:: exit(1);
    }
}





















































































































































































































































































































