mod pa;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use pa::Pa;
use std::thread;
use std::sync::{Arc,Mutex};

struct Hub{
    nodes: HashMap<usize, TcpStream>,
    next: usize,
}

impl Hub {
    pub fn new()-> Self {
        Hub { nodes: HashMap::new(), next: 1 as usize }
    }

    pub fn add_node(&mut self, stream: TcpStream) -> usize{

        self.nodes.insert(self.next, stream);
        self.next+=1;
        return self.next-1;
    }    

    pub fn get_node(&mut self, node_id: usize) -> &mut TcpStream {
        self.nodes.get_mut(&node_id).unwrap()
    }
    
 
}

fn handle_client(nodes: Arc<Mutex<Hub>>, node_id: usize){
    let mut node_stream = nodes.lock().unwrap().get_node(node_id).try_clone().unwrap();
    loop{
    let mut buffer = [0 as u8; 1024];
    match node_stream.read(&mut buffer) {
        Ok(0) => {},
        Ok(size) => {
            let b = buffer[0] as char;
            let send_to_node_id: usize = b.to_digit(10).unwrap() as usize;
            println!("{:#?}", send_to_node_id);
            let mut send_to_node: TcpStream = nodes.lock().unwrap().get_node(send_to_node_id).try_clone().unwrap();
            match send_to_node.write(&buffer[0..size]) {
                Ok(_) => {
                    println!("Sent data!");
                },
                Err(e) => panic!("Unable to write to stream! {}",e),
            }
        },
        Err(e) => panic!("Error while reading from stream {}", e),
    }
    }
}



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
    let port_and_addr: Pa = handle_args(args).unwrap();
    let nodes = Arc::new(Mutex::new(Hub::new()));

    let listener: TcpListener = TcpListener::bind(port_and_addr.get_string().as_str()).expect("Unable to bind to port");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let node_id = nodes.lock().unwrap().add_node(stream);
                println!("Hello ...");
                let nodes_clone  = Arc::clone(&nodes);
                let node_thread = thread::spawn(move || {
                    handle_client(nodes_clone, node_id);
                });
            },
            Err(e) => println!("Error : {}", e),
        }
    }
}
