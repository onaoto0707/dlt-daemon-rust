extern crate libc;
extern crate nix;

use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use nix::poll::*;
use std::io::{BufRead, BufReader, BufWriter, Write};

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let f = OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK).open("/tmp/dlt").unwrap();
    //let temp = OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open("/tmp/dlt").unwrap();
    let d = f.as_raw_fd();
    let mut fds = [PollFd::new(d, PollFlags::POLLIN)];
    let mut b = String::new();
    let mut reader = BufReader::new(f);

    thread::spawn(|| {
        let temp = OpenOptions::new().write(true).open("/tmp/dlt").unwrap();
        let mut writer = BufWriter::new(temp);
        let b  = b"test";
        loop {
            thread::sleep(Duration::from_secs(5));
            println!("thread loop");
            let byte = writer.write_all(b).unwrap();
            writer.flush().unwrap();
        }
    });

    loop {
        println!("begin");
        thread::sleep(Duration::from_secs(5));
        let data = reader.read_line(&mut b).unwrap();
        if data != 0 {
            println!("data: {}", b);
            b.clear();
        }
        //let nfds = poll(&mut fds, -1).unwrap();
        //if fds[0].revents().unwrap().contains(PollFlags::POLLIN) {
        //    let data = reader.read_line(&mut b).unwrap();
        //    if data != 0 {
        //        println!("data: {}", b);
        //        b.clear();
        //    }
        //}
        else
        {
            println!("Nothing to do");
            break;
        }
        println!("loop");
    }
    
}
