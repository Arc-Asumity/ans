// Copyright (c) 2025 Arc Asumity
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/arg.rs
// This module handles command-line argument parsing.

use clap::{Arg, ArgAction, Command};

fn spawn_common_help(cmd: Command) -> Command {
    cmd
        .after_help("Author: Arc Asumity <arcasumity@hotmail.com>\n\nLicense:\n  Copyright (c) 2025 Arc Asumity\n  Licensed under the GPLv3 or later License.")
        .arg_required_else_help(true)
}

fn build_cli() -> Command {
    spawn_common_help(
        Command::new("Ans")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("This is a DNS server.")
            .disable_version_flag(true)
            // Version
            .arg(
                Arg::new("version")
                    .long("version")
                    .short('V')
                    .help("Show version information")
                    .action(ArgAction::SetTrue),
            ),
    )
}

pub fn handle_cli() {
    let matches = build_cli().get_matches();
    if matches.get_flag("version") {
        println!(
            "Ans DNS Server Version {}\nCopyright (c) 2025 Arc Asumity\nLicensed under the GPLv3 or later License.",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    match matches.subcommand() {
        _ => {
            println!("[config]: Unknown Arg");
        }
    }
}
