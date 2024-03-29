extern crate ws;

use ws::{Handler, Sender, Result, Message, CloseCode};

pub mod oscsender;

pub struct Server{
	pub wssender: Sender,
    pub sender: oscsender::Sender
}

impl Handler for Server{

    fn on_message(&mut self, msg: Message) -> Result<()> {

   		println!("message received");
        self.sender.send();
        // Echo the message back
        self.wssender.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}