mod pa;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use pa::Pa;
use std::thread;

fn handle_args(args: Vec<String>) -> Result<Pa, String> {
    match args.len() {
        1 => Ok(Pa::new(String::from("0.0.0.0"),String::from("8080"))),
        2 => Ok(Pa::new(String::from("0.0.0.0"),args[1].to_string())),
        3 => Ok(Pa::new(args[1].to_string(), args[2].to_string())),
        _ => Err(String::from("Unable to parse address and port")),
    }
}

fn handle_client(mut stream: TcpStream){
    thread::spawn(|| {
        loop {}   
    });
    thread::spawn(|| {
        loop {}   
    });
}

struct hub{
    nodes: HashMap<usize, TcpStream>,
}

impl hub {
    pub fn new()-> Self {
        hub { nodes: HashMap::new() }
    }

    pub fn add_node(&mut self, node_id: usize, stream: TcpStream){
        self.nodes.insert(node_id, stream);
        self.nodes.in
    }    

    pub fn delete_node(&mut self, node_id: usize){
        self.nodes.remove(&node_id);
    }    
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let port_and_addr: Pa = handle_args(args).unwrap();
    let mut nodes_str = String::new();
    match std::io::stdin().read_to_string(&mut nodes_str){
        Ok(_) => {},
        Err(e) => println!("Error while reading number of nodes"),
    }
    
    let nodes = nodes_str.parse::<usize>().unwrap();
    


    let listener: TcpListener = TcpListener::bind(port_and_addr.get_string().as_str()).expect("Unable to bind to port");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream),
            Err(e) => println!("Error : {}", e),
        }
    }
}
