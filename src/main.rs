use std::net::UdpSocket;
use std::str;

fn main() {
    println!("Tepostli 0.1.0");
    match UdpSocket::bind("0.0.0.0:0") {
    	Ok(socket) => {
    		match socket.send_to("(init Tepostli (version 15.1))".as_bytes(), "127.0.0.1:6000") {
    			Ok(size) => {
    				println!("Sent {}", size);
    				let mut i = 0;
    				loop {
	    				let mut buf = [0; 4096];
	    				match socket.recv_from(&mut buf) {
	    					Ok((bytes, _)) => {
	    						match str::from_utf8(&buf) {
	    							Ok(message) => {
	    								println!("Received {}", bytes);
	    								println!("Message {}", message);
	    							}
	    							Err(e) => panic!("Could not read string {}", e)
	    						}
	    					}
	    					Err(e) => panic!("Could not receive {}", e)
	    				}
	    				i = i + 1;
	    				if i > 100 {
	    					break;
	    				}
    				}
    			}
    			Err(e) => panic!("Could not send {}", e)
    		}
    	}
    	Err(e) => panic!("Could not bind {}", e)
    }
}
