use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, sync::broadcast, net::TcpListener};



#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8088").await.unwrap(); // Bind to localhost on port 8088

    let (tx, _rx) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap(); // Accept incoming connections

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn( async move { 
            let (reader, mut writer) = socket.split(); // Split the socket into a reader and a writer

            let mut reader = BufReader::new(reader); // Create a buffered reader for the socket
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap(); // Write back the data received
                        }
                    }
                }
            }
        });
    } 
}