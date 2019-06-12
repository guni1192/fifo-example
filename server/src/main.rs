use std::path::Path;
use std::str;

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd;

use fifoconfig::SERVER_FIFO_PATH;

fn main() -> nix::Result<()> {
    let server_fifo_path = Path::new(&SERVER_FIFO_PATH);
    let mode = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IWGRP;
    match unistd::mkfifo(server_fifo_path, mode) {
        Ok(_) => println!("[INFO] created {}", SERVER_FIFO_PATH),
        Err(err) => println!("[ERROR] creating fifo: {}", err),
    }

    let server_fd = open(server_fifo_path, OFlag::O_RDONLY, mode)?;
    let _dummy_fd = open(server_fifo_path, OFlag::O_WRONLY, mode)?;

    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        match unistd::read(server_fd, &mut buf) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("[ERROR]: read server_fd: {}, FIFO: {}", e, SERVER_FIFO_PATH);
                continue;
            }
        }
        unistd::close(server_fd)?;
        let message = match str::from_utf8(&buf) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("[ERROR] convert [u8] to str: {}", e);
                continue;
            }
        };
        println!("[INFO] recive message: {}", message);
        if message == "exit" {
            break;
        }
    }

    Ok(())
}
