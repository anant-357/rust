use std::time::{SystemTime,Duration};
use std::sync::mpsc::{Sender, Receiver};
pub struct Timer {
   interval: Duration,
   sender: Sender<usize>,
} 

impl Timer {
    pub fn new(interval: Duration, sender: Sender<usize>) -> Self{
        Timer {
            interval,
            sender
        }
    }

    pub fn start(&self, rx: Receiver<usize>){
        let start = SystemTime::now();
        loop {
            if rx.recv() == Ok(1) {
                break;
            }
            else if self.interval >= start.elapsed().unwrap() {
                self.sender.send(1);
                break;
            }
        }
    }

}

