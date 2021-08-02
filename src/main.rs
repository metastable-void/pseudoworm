extern crate pty;
//extern crate libc;

use std::io::Read;
use std::io::Write;
use std::process::{Command};
use std::os::unix::process::CommandExt;
use std::thread::sleep;
use std::time::Duration;

use pty::fork::*;

fn main() {
    let fork = Fork::from_ptmx().unwrap();

    if let Some(mut master) = fork.is_parent().ok() {
        // Read output via PTY master
        let mut output = String::new();

        match master.unlockpt() {
            Ok(_fd) => println!("unlockpt()"),
            Err(e) => panic!("error in unlockpt(): {}", e),
        }

        sleep(Duration::from_secs(1));

        let mut write_buffer: Vec<u8>;
        write_buffer = vec![0x70, 0x73, 0x0a];

        match master.write(&write_buffer) {
            Ok(_size) => println!("written"),
            Err(_) => panic!("write error"),
        }

        match master.read_to_string(&mut output) {
            Ok(_nread) => println!("{}", output),
            Err(e)     => panic!("read error: {}", e),
        }
    }
    else {
        // Child process
        if let Ok(mut child) = Command::new("bash").arg0("-bash").spawn() {
            sleep(Duration::from_secs(2));
            child.kill().expect("child wasn't running");
        } else {
            println!("could not execute command");
        }
    }
}
