use std::net::UdpSocket;
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;

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

fn get_player_types(slist: &Vec<sparser::SList>, default: u16) -> u16 {
	let player_types = sparser::find_list(slist, "player_types".to_string());
    match player_types {
    	Some(player_types_list) => {
			if player_types_list.len() > 1 {
				match player_types_list[1] {
					sparser::SList::Atom(ref string) => {
						match string.parse::<u16>() {
							Ok(value) => return value,
							Err(_) => return default
						}
					},
					_ => return default
				}
			} else {
				return default
			}
		},
		None => return default
	}
}

fn main() {
    println!("Tepostli 0.1.0");
    let port: u16;
    match UdpSocket::bind("0.0.0.0:0") {
    	Ok(socket) => {
    		match socket.send_to("(init Ocelotl (version 15.1))".as_bytes(), "127.0.0.1:6000") {
    			Ok(_) => {
    				let (_, next_port) = receive(&socket); // init
    				port = next_port;
    				receive(&socket); // server parameters
    				let (player_param, _) = receive(&socket); // player parameters
    				let player_param_slist = sparser::sparser(player_param);
    				let player_types = get_player_types(&player_param_slist, 10);
    				println!("Types {}", player_types);
    				let mut i = 1;
    				loop {
    					// right now we just consume the message
    					receive(&socket);
    					i = i + 1;
    					if i > player_types {
    						break;
    					}
    				}
    			},
    			Err(e) => panic!("Could not send {}", e)
    		};
    		let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    		thread::spawn(move || {
    			loop {
    				let (message, _) = receive(&socket);
    				tx.send(message).unwrap();
    			}
    		});
    		let mut simulation_time = 0;
    		loop {
    			let message = rx.recv().unwrap();
    			let slist = sparser::sparser(message);
    			if slist.len() > 0 {
    				let head = &slist[0];
    				match head {
    					&sparser::SList::Atom(ref string) if *string == "sense_body".to_string() => {
    						loop {
    							match rx.recv_timeout(Duration::from_millis(10)) {
    								Ok(message) => println!("Message: {}", message),
    								Err(_) => {
    									println!("No more messages");
    									break;
    								}
    							}
    						}
    						println!("Cycle {}", simulation_time);
    						simulation_time = simulation_time + 1;
    					},
    					_ => continue
    				}
    			}
    		}
    	}
    	Err(e) => panic!("Could not bind {}", e)
    }
}
