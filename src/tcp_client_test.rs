use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> io::Result<()> {
    // Connect to the server
    let stream = TcpStream::connect("localhost:8088")?;
    println!("Connected to server!");

    let mut reader_stream = stream.try_clone()?;
    thread::spawn(move || {
        let mut reader = BufReader::new(&reader_stream);
        let mut buffer = String::new();
        loop {
            match reader.read_line(&mut buffer) {
                Ok(_) => {
                    println!("Client 2: {}", buffer);
                    buffer.clear();
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                    break;
                }
            }
        }
    });

    let mut writer_stream = stream;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        writer_stream.write_all(input.as_bytes())?;
        println!("You: {}", input);
    }
}