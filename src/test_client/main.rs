// A WebSocket client that sends one message then closes
extern crate ws;

use ws::{connect, CloseCode};

fn main() {
  connect("ws://127.0.0.1:9000", |out| {
      out.send("Hello WebSocket").unwrap();

      move |msg| {
          println!("Got message: {}", msg);
          out.close(CloseCode::Normal)
      }
  }).unwrap()
} 