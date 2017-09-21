use std::net::UdpSocket;
use std::str;

mod sparser;

fn receive(socket: &UdpSocket) -> String {
	let mut buf = [0; 4096];
	match socket.recv_from(&mut buf) {
		Ok((_, _)) => {
			match str::from_utf8(&buf) {
				Ok(message) => {
					return message.to_string();
				}
				Err(_) => return "(error could_not_read)".to_string()
			}
		}
		Err(_) => return "(error could_not_read)".to_string()
	}
}

fn main() {
    println!("Tepostli 0.1.0");
    match UdpSocket::bind("0.0.0.0:0") {
    	Ok(socket) => {
    		match socket.send_to("(init Ocelotl (version 15.1))".as_bytes(), "127.0.0.1:6000") {
    			Ok(_) => {
    				let mut i = 0;
    				loop {
	    				let response = receive(&socket);
						let slist = sparser::sparser(response);
						if slist.len() > 0 {
							match slist[0] {
								sparser::SList::Atom(ref string) => println!("Received {}", string),
								_ => println!("Expected SList::Atom")
							}
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
