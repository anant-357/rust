mod pa;
mod frame;
mod timer;
use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, time::Duration, path::Path, fs::OpenOptions, sync::{mpsc::channel, Mutex, Arc}, thread};
use pa::Pa;
use crate::frame::Frame;
use crate::timer::Timer;



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
    let mut file = OpenOptions::new().read(true).open(Path::new("alice29.txt")).unwrap();
    let (tx, rx) = channel::<usize>();
    let sn = Arc::from(Mutex::from(0 as usize));
    let mut read_stream = stream.try_clone().unwrap();
    let backup_stream = stream.try_clone().unwrap();
    let (timer_tx, timer_rx) = channel::<usize>();
    thread::spawn(move||loop {
        match rx.recv().unwrap() {
            1 => {},
            2 =>{
                let sender_clone = tx.clone();
                let timer = Timer::new(Duration::from_secs(1), sender_clone);
                thread::spawn(move || {timer.start(timer_rx.into_iter);});
            },
            _ => {},
        }            
    });
    loop {
        let mut buffer = [0 as u8; 5000];
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let frame: Frame = Frame::new(*sn.lock().unwrap(), String::from_utf8(Vec::from_iter(buffer)).unwrap());               
                let bytes: &[u8] = unsafe { frame.any_as_u8_slice::<Frame>() };
                stream.write(&bytes);
                let mut ack_buffer = [0 as u8; 1024];
                match read_stream.read(&mut ack_buffer){
                    Ok(size) => {
                        let ack = unsafe {Frame::u8_slice_as_any::<Frame>(&ack_buffer[0..size])};
                        if ack.get_sequence_number().unwrap() == 1 - *sn.lock().unwrap() {
                            timer_tx.send(1);
                            let now:usize = *sn.lock().unwrap();
                            *sn.lock().unwrap() = 1 - now;
                        }
                    },
                    Err(e) => panic!("Error while reading from stream {}",e),
                }
            },
            Err(e) => panic!("Error while reading from file {}", e),
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
