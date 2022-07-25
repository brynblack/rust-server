use std::{
    env::args,
    io::{Read, Result},
    net::TcpListener,
};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let listener = TcpListener::bind(&args[1])?;
    for stream in listener.incoming() {
        let mut buffer = String::new();
        stream?.read_to_string(&mut buffer)?;
        println!("{}", buffer);
    }
    Ok(())
}
