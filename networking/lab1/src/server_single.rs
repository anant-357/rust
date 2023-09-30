mod pa;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use pa::Pa;
use std::process::Command;
use std::str;

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
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer[..]) {
            Ok(size) => {
                let out = Command::new("man").arg("-f").arg(str::from_utf8(&buffer[0..size-1]).unwrap()).output().expect("failed to execute");
                match out.status.code() {
                    Some(0) => {stream.write(String::from_utf8_lossy(&out.stdout).as_bytes()).unwrap();},
                    _ =>  {stream.write(String::from("No such command").as_bytes()).unwrap();},
                }
            },
            Err(e) => {
                println!("Error : {}", e);
                stream.shutdown(Shutdown::Both).unwrap();
            }
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
