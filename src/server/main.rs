use ws::listen;

mod wsserver;

static WS_MY_IP: &'static str = "127.0.0.1:9000";
static OSC_MY_IP: &'static str = "127.0.0.1:56789";
static OSC_DEST_IP: &'static str = "127.0.0.1:8000";

fn main() {
    listen(WS_MY_IP, |wssender| {
        wsserver::Server { 
            wssender: wssender,
            sender: wsserver::oscsender::Sender::new(OSC_MY_IP, OSC_DEST_IP)
         } 
    }).unwrap()
}
