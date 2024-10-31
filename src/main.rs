use std::net::UdpSocket;

use gdl90::decoder::*;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:4000").unwrap();
    let mut buf = [0; 2048];
    let mut decoder = Gdl90Decoder::new();

    loop {
        // Receives a single datagram message on the socket.
	    // If `buf` is too small to hold
        // the message, it will be cut off.
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        // Redeclare `buf` as slice of the received data
	    // and send data back to origin.
        let buf = &mut buf[..amt];
        if let DecodeResult::Complete(messages) = decoder.decode_bytes(&buf) {
            for message in messages {
                match message {
                    Err(e) => println!("{:?}", e),
                    Ok(message) => println!("{:?}", message)
                }
            }
        }
        
    }
}