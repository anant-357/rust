mod pa;
use std::net::TcpStream;
use std::io::{Write, Read};
use pa::Pa;
use std::{str, thread};

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
    println!("1");
    match TcpStream::connect(port_and_address.get_string().as_str()) {
        Ok(mut stream) => {
            let mut read_stream = stream.try_clone().unwrap();
    println!("2");
            thread::spawn(move || {
                println!("Started Reading!");
                let mut buffer = [0 as u8;1024];
                loop{match read_stream.read(&mut buffer) {
                    Ok(0) => {},
                    Ok(size) => {
                        println!("from server: {}", str::from_utf8(&buffer[0..size]).unwrap());
                    },
                    Err(e) => panic!("Unable to read from stream {}", e),
                }}
            });

                println!("Can write!");
                loop{
                let mut node_id: String = String::new();
                println!("Enter node to send data to :");
                ::std::io::stdin().read_line(&mut node_id).unwrap();
                let mut data: String = String::new();
                println!("Enter data :");
                ::std::io::stdin().read_line(&mut data).unwrap();
                let buffer = node_id + data.as_str();
                match stream.write(buffer.as_bytes()){
                    Ok(_) => {println!("WRITTEN TO STREAM!");},
                    Err(_) => panic!("Unable to write to stream!"),
                }
                }
        },
        Err(e) => println!("Failed to connect {}", e),
    }
    println!("3");

}
