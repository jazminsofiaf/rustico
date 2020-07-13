use std::sync::mpsc::{Receiver};
use std::thread;
use crate::consts::consts::consts::STOP_LOGGING_MSG;
use std::thread::JoinHandle;
use std::fs::File;
use std::io::Write;

pub struct Logger {}

impl Logger {
    pub fn new(should_i_log: bool, debug_msg_receiver: Receiver<String>) -> JoinHandle<()> {
        println!("in log msgs!");
        let child = thread::spawn(move || {
            println!("spawned thread");
            let mut file = File::create("log.rustico").expect("error opening log file");
            loop {
                println!("in loop");
                let current_msg = debug_msg_receiver.recv().expect("error reading");

                file.write_all(current_msg.as_ref()).expect("error writing log file");
                file.write_all("\n".as_ref()).expect("error writing log file");
                if current_msg.as_str() == STOP_LOGGING_MSG {
                    break;
                }
                if should_i_log {
                    println!("{}", format!("[DEBUG] {}", current_msg));
                }
            }
        });
        return child;
    }
}
