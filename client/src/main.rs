use std::{env::args, io::Write, net::TcpStream};

fn main() {
    let args: Vec<String> = args().collect();
    TcpStream::connect(&args[1])
        .unwrap()
        .write(&args[2].as_bytes())
        .unwrap();
}
