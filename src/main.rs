/*
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//! Oxi is a very basic CLI for the headless Oxidation torrent client.
//!
//! Oxi can operate in two modes, the first of which is the
//! primary focus of Oxi's current development:
//! 	* Immediate
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

extern crate oxi;

use oxi::Config;

fn main() {
    let config = match Config::new(&mut std::env::args()) {
        Ok(c) => {
            if c.help_triggered {
                return;
            } else {
                c
            }
        }
        Err(e) => {
            eprintln!("Bad argument listing: {}", e);
            std::process::exit(1);
        }
    };

    std::process::exit(match oxi::run(&config) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Something bad happaned: {}", e);
            1
        }
    });
}
