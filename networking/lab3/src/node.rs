mod pa;
use std::net::TcpStream;
use std::io::{Write, Read};
use pa::Pa;
use std::str;
use std::path::Path;
use std::fs::OpenOptions;

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
            let mut file = OpenOptions::new().read(true).open(Path::new("alice29.txt")).unwrap();
            loop {
                let mut buffer = [0 as u8; 5000];
                match file.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(_) => {},
                    Err(e) => panic!("error while reading from file : {}",e),
                }
                match stream.write(&buffer) {
                    Ok(_) => {
                        let mut ack_buffer = [0 as u8; 96];
                        match stream.read(&mut ack_buffer) {
                            Ok(ack_size) => {
                                if str::from_utf8(&ack_buffer[0..ack_size]).unwrap() == "ACK" {
                                    println!("Received Ack !");
                                } else {
                                    println!("Didnot recieve ack !");
                                }
                            },
                            Err(e) => println!("Unable to read ack from stream {}", e),
                        }
                    },
                    Err(e) => println!("Unable to write to stream {} ", e),
                }
            }  
        },
        Err(e) => println!("Failed to connect {}", e),
    }

}
