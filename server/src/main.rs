use std::{env::args, io::Read, net::TcpListener};

fn main() {
    TcpListener::bind("0.0.0.0:".to_owned() + &args().nth(1).unwrap())
        .unwrap()
        .incoming()
        .for_each(|stream| {
            let mut buffer = String::new();
            stream.unwrap().read_to_string(&mut buffer).unwrap();
            println!("{}", buffer);
        });
}
