use std::{
    io::{BufReader, Read, Result, Write},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    thread,
};
mod server;
use server::requests::handle_request;
fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = [0; 4096];
    match reader.read(&mut buffer) {
        Ok(n) => {
            println!("Received {} bytes: {:?}", n, from_utf8(&buffer[..n]));
            let response: String = handle_request(from_utf8(&buffer[..n]).unwrap());
            let response_bytes = response.as_bytes();
            stream.write_all(response_bytes)?;
            stream.flush()?;
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        thread::spawn(|| {
            handle_connection(stream).unwrap_or_else(|error| {
                eprintln!("Error handling connection: {}", error);
            });
        });
    }
    Ok(())
}
