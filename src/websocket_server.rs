/* Hosts the websocket server that the client connects to */

use actix_web::{web, App, HttpServer, HttpRequest, Responder, Error};
use actix_web_actors::ws;


struct MyWebSocket;

impl ws::Handler for MyWebSocket {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // Handle WebSocket messages
        Ok(())
    }
}

