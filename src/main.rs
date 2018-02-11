extern crate oxi;
extern crate regex;

use regex::Regex;
use oxi::Config;



fn main() {

	let config = match Config::new(&mut std::env::args()) {
		Ok(c) => {
			if c.help_triggered {
				return;
			} else {
				c
			}
		},
		Err(e) => {
			eprintln!("Bad argument listing: {}", e);
			return;
		},
	};


	std::process::exit(match oxi::run(config) {
		Ok(_) => 0,
		Err(e) => {
			eprintln!("Something bad happaned: {}", e);
			1
		}
	});
}
