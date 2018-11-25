/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Oxi is a very basic CLI for the headless Oxidation torrent client.
//!
//! Oxi can operate in two modes, the first of which is the
//! primary focus of Oxi's current development:
//! * Immediate
//!	* Active
//!
//! # Immediate
//!
//! Immediate mode is the most bare-bones CLI interface. It's
//! completely stateless, and nothing is continally active.
//! Every query is a new connection to the Oxidation server,
//! with no practical state conserved from run to run, apart
//! from minimal caching.
//!
//! # Active
//!
//! Active mode is planned to be a TUI written with ncurses,
//! which maintains an active connection to the Oxidation server
//! in order to continually update its state.

#![cfg_attr(feature = "cargo-clippy", deny(pedantic))]

extern crate bufstream;
extern crate oxidant;

use bufstream::BufStream;
use std::io::BufRead;
use std::io::Write;
use std::net::TcpStream;

/// Config acts as a buffer for most of the information we're going
/// to parse out at the beginning of the app's lifecycle.
pub struct Config {
    /// The name of the host to connect to
    hostname: String,
    /// The port to connect to the host on
    port: String,
    /// The command to send the host
    command: Option<oxidant::Command>,
    /// Whether or not we should exit early because of a help request
    pub help_triggered: bool,
}

impl Config {
    /// Creates a new `Config` object from the program's arguments.
    ///
    /// # Arguments
    ///
    /// `args` - the arguments passed in upon running the application.
    ///
    /// # Panics
    ///
    /// This function panics if:
    /// 	* For some reason, the binary name isn't passed as an argument.
    ///
    /// # Errors
    /// This function returns an error if:
    /// 	* A multi-part argument ended prematurely
    ///		* A command was not passed to the application.
    pub fn new(args: &mut std::env::Args) -> Result<Self, &'static str> {
        let mut hostname = String::new();
        let mut port = String::new();
        let mut help_triggered = false;

        let bin = args
            .next()
            .expect("Yikes, we don't even have the binary name? Something is definitely wrong...");
        let mut cmd: Vec<String> = Vec::new();
        while let Some(s) = args.next() {
            match s.trim() {
                "-H" | "--hostname" => {
                    hostname = args.next().ok_or("argument ended prematurely")?;
                }
                "-P" | "--port" => {
                    port = args.next().ok_or("argument ended prematurely")?;
                }
                "-h" | "-?" | "--help" => {
                    print_help(&bin);
                    help_triggered = true;
                }
                o => {
                    cmd.push(String::from(o));
                }
            }
        }

        let command = match oxidant::Command::parse(&cmd) {
            Ok(c) => Some(c),
            Err(_e) => None,
        };

        if !help_triggered && command == None {
            print_help(&bin);
            help_triggered = true;
        }

        if hostname.is_empty() {
            hostname = "127.0.0.1".to_string();
        }

        if port.is_empty() {
            port = "4422".to_string();
        }

        Ok(Self {
            hostname,
            port,
            help_triggered,
            command,
        })
    }
}

/// Prints a help prompt with the proper formatting and binary name
///
/// # Arguments
///
/// `bin` - the name of the binary currently being executed
fn print_help(bin: &str) {
    println!("oxi v.0.0.1 - (C) 2018 Oxidation Team. All rights reserved.");
    println!(
        "usage: {} [-H|--hostname hostname] [-P|--port port] <command> <args>",
        bin
    );
}

/// Acts as the main thread for the program.
///
/// TODO: Add more to this.
pub fn run(config: &Config) -> Result<(), &'static str> {
    let address = format!("{}:{}", config.hostname, config.port);
    let conn = match TcpStream::connect(address) {
        Ok(c) => c,
        Err(_) => return Err("unable to connect to server"),
    };
    let mut stream = BufStream::new(conn);

    let command = match config.command {
        Some(ref c) => c.serialize(),
        None => panic!("command not found and help not triggered!"),
    };

    match stream.write_all(command.as_bytes()) {
        Ok(_) => {}
        Err(_) => return Err("problem sending command"),
    }

    match stream.flush() {
        Ok(_) => {}
        Err(_) => return Err("problem flushing stream"),
    }

    let mut resp = String::new();

    match stream.read_line(&mut resp) {
        Ok(_) => {}
        Err(_) => return Err("problem reading line from server"),
    }

    println!("{}", resp.trim());
    Ok(())
}
