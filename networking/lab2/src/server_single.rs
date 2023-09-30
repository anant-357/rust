mod pa;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use pa::Pa;
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

fn handle_client(mut stream: TcpStream){
    println!("Connected!");
    let mut file = OpenOptions::new().create(true).write(true).open(Path::new("out.txt")).unwrap();
    loop {
        let mut buffer = [0 as u8; 5000];
        match stream.read(&mut buffer) {
            Ok(_) => {
                match file.write_all(&buffer) {
                    Ok(_) => println!("Wrote"),
                    Err(e) => panic!("Error while writing to file {}", e),
                }
                stream.write(b"ACK").expect("Unable to send ack");
            },
            Err(e) => panic!("Error while reading {}",e),
        }
        
    }   
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let port_and_addr: Pa = handle_args(args).unwrap();

    let listener: TcpListener = TcpListener::bind(port_and_addr.get_string().as_str()).expect("Unable to bind to port");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Error : {}", e),
        }
    }
}
