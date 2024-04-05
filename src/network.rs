/* handle communication with the server using HTTP requests and WebSockets. 
contains functions to send and receive messages, authenticate users */

use reqwest::Client;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tungstenite::connect;
use url::Url;

pub struct Network {
    client: Client,
    websocket_sender: Sender<String>,
    websocket_receiver: Arc<Mutex<Receiver<String>>>,
}

impl Network {
    pub async fn new() -> Result<Self, reqwest::Error> {
        // reqwest client
        let client = Client::new();

        let (websocket_sender, websocket_receiver) = Network::establish_websocket().await?;

        Ok(Network {
            client,
            websocket_sender,
            websocket_receiver: Arc::new(Mutex::new(websocket_receiver)),
        })
    }

    // need to find a proper way to authenticate: use IllinoisNet?
    // i got this from chat gpt
    pub async fn authenticate_user(&self, username: &str, password: &str) -> Result<String, reqwest::Error> {
        // send a POST request for authentication?
        let response = self.client.post("https://bhailang.js.org")
            .json(&json!({
                "username": username,
                "password": password,
            }))
            .send()
            .await?;

        response.text().await
    }

    async fn establish_websocket() -> Result<(Sender<String>, Receiver<String>), tungstenite::Error> {
        let (tx, rx) = mpsc::channel::<String>(100); // reveiving channel
        let (websocket_tx, websocket_rx) = mpsc::channel::<String>(100); // sending channel

        let ws_url = Url::parse("ws://example.com/ws").expect("Invalid WebSocket URL");

        let (ws_stream, _) = connect(ws_url).await?;
        let (write, read) = ws_stream.split();


        tokio::spawn(Self::handle_websocket_messages(read, tx.clone()));

        // Spawn a task to send WebSocket messages
        tokio::spawn(Self::send_websocket_messages(write, websocket_rx));

        Ok((websocket_tx, rx))
    }

    async fn handle_websocket_messages(mut read: tungstenite::protocol::WebSocketStream<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>, tx: Sender<String>) {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(tungstenite::Message::Text(text)) => {
                    // Send received message through channel
                    if let Err(_) = tx.send(text).await {
                        // Handle channel send error
                    }
                }
                Ok(_) => {
                    // Handle other WebSocket message types (Binary, Close, Ping, Pong)
                }
                Err(_) => {
                    // Handle WebSocket receive error
                }
            }
        }
    }

    async fn send_websocket_messages(mut write: tungstenite::protocol::WebSocketSink<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>, rx: Receiver<String>) {
        while let Some(message) = rx.recv().await {
            if let Err(_) = write.send(tungstenite::Message::Text(message)).await {
                // Handle WebSocket send error
            }
        }
    }

    pub async fn send_message(&self, message: &str) -> Result<(), mpsc::error::SendError<String>> {
        self.websocket_sender.send(message.to_string()).await
    }

    pub async fn receive_message(&self) -> Option<String> {
        let mut receiver = self.websocket_receiver.lock().unwrap();
        receiver.recv().await
    }
}

