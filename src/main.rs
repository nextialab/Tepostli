use std::net::UdpSocket;
use std::str;

mod sparser;

fn receive(socket: &UdpSocket) -> (String, u16) {
	let mut buf = [0; 4096];
	match socket.recv_from(&mut buf) {
		Ok((_, addr)) => {
			match str::from_utf8(&buf) {
				Ok(message) => {
					return (message.to_string(), addr.port());
				}
				Err(_) => return ("(error could_not_read)".to_string(), addr.port())
			}
		}
		Err(_) => return ("(error could_not_read)".to_string(), 0)
	}
}

fn main() {
    println!("Tepostli 0.1.0");
    let mut next_port: u16 = 0;
    match UdpSocket::bind("0.0.0.0:0") {
    	Ok(socket) => {
    		match socket.send_to("(init Ocelotl (version 15.1))".as_bytes(), "127.0.0.1:6000") {
    			Ok(_) => {
    				let (_, port) = receive(&socket); // init
    				let (_, _) = receive(&socket); // server parameters
    				let (player_param, _) = receive(&socket); // player parameters
    				let player_param_slist = sparser::sparser(player_param);
    				let player_types = sparser::find_list(&player_param_slist, "player_types".to_string());
    				match player_types {
    					Some(player_types_list) => {
    						if player_types_list.len() > 1 {
	    						match player_types_list[1] {
	    							sparser::SList::Atom(ref string) => println!("Player types {}", string),
	    							_ => println!("Expected SList::Atom")
	    						}
    						}
    					},
    					None => println!("Not found")
    				}
    				/*let mut i = 0;
    				loop {
	    				let (response, port) = receive(&socket);
	    				next_port = port;
	    				println!("Raw {}", response);
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
    				}*/
    			},
    			Err(e) => panic!("Could not send {}", e)
    		};
    		println!("Next port {}", next_port);
    		/*match socket.send_to("(move -10 0)".as_bytes(), format!("127.0.0.1:{}", next_port)) {
    			Ok(_) => {
    				println!("agent moved!");
    			},
    			Err(e) => panic!("Could not send {}", e)
    		}*/
    	}
    	Err(e) => panic!("Could not bind {}", e)
    }
}
