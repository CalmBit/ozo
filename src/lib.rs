extern crate regex;

use regex::Regex;

use std::net::{TcpStream};
use std::io::Read;

pub struct Config {
	hostname: String,
	port: String,
	pub help_triggered: bool,
}

impl Config {
	pub fn new(args: &mut std::env::Args) -> Result<Config, &'static str> {
		let mut hostname = String::new();
		let mut port = String::new();
		let mut help_triggered = false;

		let bin = args.next().expect("Yikes, we don't even have the binary name? Something is definitely wrong...");
		loop {
			match args.next() {
				Some(s) => match s.trim() {
					"-H" | "--hostname" => {
						hostname = args.next().ok_or("argument ended prematurely")?;
					},
					"-P" | "--port" => {
						port = args.next().ok_or("argument ended prematurely")?;
					},
					"-h" | "-?" | "--help" => {
						print_help(&bin);
						help_triggered = true;
					}
					&_ => eprintln!("invalid argument encountered")
				},
				None => {break;}
			}
		}
		
		if hostname.is_empty() {
			hostname = "127.0.0.1".to_string();
		}

		if port.is_empty() {
			port = "4422".to_string();
		}

		Ok(Config {
			hostname,
			port,
			help_triggered
		})
	}
}

fn print_help(bin: &String) {
	println!("oxi v.0.0.1 - (C) 2018 Ethan Brooks. All rights reserved.");
	println!("usage: {} [-H|--hostname hostname] [-P|--port port]", bin);
}

pub fn run(config: Config) -> Result<(), &'static str> {
	let address = format!("{}:{}", config.hostname, config.port);
	let mut conn = match TcpStream::connect(address) {
		Ok(c) => c,
		Err(e) => return Err("unable to connect to server")
	};
	let mut resp = String::new();
	conn.read_to_string(&mut resp).unwrap();
	println!("{}", resp);
	Ok(())
}
