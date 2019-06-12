pub static CLIENT_FIFO_PATH: &'static str = "/tmp/fifo-client";
pub static SERVER_FIFO_PATH: &'static str = "/tmp/fifo-server";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
