use std::path::Path;

use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd;

use fifoconfig::SERVER_FIFO_PATH;

fn main() -> nix::Result<()> {
    let server_fifo_path = Path::new(&SERVER_FIFO_PATH);
    let mode = Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IWGRP;
    let server_fd = open(server_fifo_path, OFlag::O_WRONLY, mode)?;
    let message = "Hello".as_bytes();

    unistd::write(server_fd, &message).expect("[ERROR]: write server_fd:");

    Ok(())
}
