use std::{io::Read, net::TcpListener};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8192")?;
    let (mut stream, _) = listener.accept()?;
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}
