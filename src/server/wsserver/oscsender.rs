extern crate rosc;

use rosc::{OscMessage, encoder, OscPacket};
use std::net::{UdpSocket};

pub struct Sender{
    dest_ip: String,
    osc_socket: UdpSocket
}

impl Sender{

    pub fn new(my_ip: &str, dest_ip: &str) -> Sender {

        let init_socket = |ip: &str|{
            match UdpSocket::bind(ip) {
                Ok(osc_socket) => osc_socket,
                Err(ex) => { 
                    panic!("Unable to bind Udp oscSocket! {}", ex);
                }
            }
        };

        Sender {
            dest_ip: dest_ip.to_string(),
            osc_socket: init_socket(&my_ip)
        }
    }

    pub fn send(&self) {

        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: "/server/started".to_string(),
            args: None,
        })).unwrap();

        match self.osc_socket.send_to(&msg_buf, &self.dest_ip) {
            Ok(_) => {},
            Err(ex) => {
                println!("Unable to send message! {}", ex);
            }
        }
    }

}
