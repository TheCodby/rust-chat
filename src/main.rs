use std::{
    io::{BufReader, Read, Result},
    net::{TcpListener, TcpStream},
    str::from_utf8,
    thread,
};
fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = [0; 4096];
    match reader.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                return Ok(());
            }
            println!("Received {} bytes: {:?}", n, from_utf8(&buffer[..n]));
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
