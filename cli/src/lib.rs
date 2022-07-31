use std::fs;
use std::error::Error;
<<<<<<< HEAD
use std::env;
=======
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

<<<<<<< HEAD
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results{
=======
    for line in search(&config.query, &contents){
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
        println!("{}",line);
    }
    
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
<<<<<<< HEAD
    pub case_sensitive: bool,
=======
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

<<<<<<< HEAD
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config { query, filename, case_sensitive });
=======
        return Ok(Config { query, filename });
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
    }
}

pub fn search<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }    
    results
}

<<<<<<< HEAD
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(query){
            results.push(line);
        }
    }    
    results
}

=======
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
#[cfg(test)]
mod tests {
    use super ::*;
    #[test]
<<<<<<< HEAD
    fn case_sensitive(){
=======
    fn one_result(){
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
<<<<<<< HEAD
Pick three.
Duct tape.";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }        
    #[test]
    fn case_insensitive(){
        let query = "rust";
        let contents = "\
Rust:
safe,fast,productive
Pick three
Trust me.
        ";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query,contents));
=======
Pick three.";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
>>>>>>> 075306f5301d2a48bfa86f7d451d009c52ab293b
    }
}