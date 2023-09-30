mod pa;
use std::net::TcpStream;
use std::io::{Write, Read};
use pa::Pa;
use std::str;

fn handle_args(args: Vec<String>) -> Result<Pa, String> {
    match args.len() {
        1 => Ok(Pa::new(String::from("0.0.0.0"),String::from("8080"))),
        2 => Ok(Pa::new(String::from("0.0.0.0"),args[1].to_string())),
        3 => Ok(Pa::new(args[1].to_string(), args[2].to_string())),
        _ => Err(String::from("Unable to parse address and port")),
    }
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let port_and_address: Pa = handle_args(args).unwrap();
    
    match TcpStream::connect(port_and_address.get_string().as_str()) {
        Ok(mut stream) => {
            println!("Connected!");
            loop { 
           let mut input : String = String::new(); 
           std::io::stdin().read_line(&mut input).unwrap();

           match stream.write(input.as_bytes()) {
                Ok(_) => {
                    let mut buffer = [0 as u8; 1024];
                    match stream.read(&mut buffer[..]) {
                        Ok(size) => {
                           println!("Meaning: {}", str::from_utf8(&buffer[0..size]).unwrap()); 
                        },
                        Err(e) => println!("Error while reading {}", e) ,
                    }
                },
                Err(_) => println!("Unable to write"),
           }
        }  
        },
        Err(e) => println!("Failed to connect {}", e),
    }

}
