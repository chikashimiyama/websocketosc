extern crate rosc;

use std::net::{UdpSocket};
use rosc::{OscPacket, OscMessage};
use ws::{listen};
use rosc::encoder;

mod wsserver;

static OSC_MY_IP: &'static str = "127.0.0.1:56789";
static OSC_DEST_IP: &'static str = "127.0.0.1:8000";
static WS_MY_IP: &'static str = "127.0.0.1:9000";

fn main() {

    let osc_socket = match UdpSocket::bind(OSC_MY_IP) {
    	Ok(osc_socket) => osc_socket,
    	Err(ex) => { 
    		panic!("Unable to bind Udp oscSocket! {}", ex);
    	}
    };

    let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
        addr: "/server/started".to_string(),
        args: None,
    })).unwrap();

    match osc_socket.send_to(&msg_buf, OSC_DEST_IP) {
    	Ok(_) => {},
    	Err(ex) => {
    		println!("Unable to send message! {}", ex);
    	}
    }

    listen(WS_MY_IP, |out| wsserver::Server { out: out } ).unwrap()

}
