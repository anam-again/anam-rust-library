#![allow(dead_code)]

use rand::prelude::*;
use std::{env, process};

use hello_rust::cli_builder::{Command, Parameter};

use crate::space_invader_generator::{GenerateSpaceInvaderInput, Offset};

mod cli_builder;
mod grid_painter;
mod ip_sniffer;
mod space_invader_generator;

fn main() {
    unsafe { env::set_var("RUST_BACKTRACE", "1") };
}

fn f1() {
    ip_sniffer::ip_sniffer();
}

fn f2() {
    let args: Vec<String> = env::args().collect();
    let command: Command = Command {
        name: "",
        desc: "",
        parameters: vec![Parameter {
            shortcut: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            flag: true,
            input: "".to_string(),
            main_argument: true,
        }],
    };
    let _processed_args = match command.process_args(&args) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            process::exit(1);
        }
    };
}

fn f3() {
    space_invader_generator::generate_space_invader(&GenerateSpaceInvaderInput {
        body_o0: Offset {
            x: rand::rng().random_range(0..5),
            y: rand::rng().random_range(0..5),
        },
        body_o1: Offset {
            x: rand::rng().random_range(0..5),
            y: rand::rng().random_range(0..5),
        },
        body_o2: Offset {
            x: rand::rng().random_range(0..5),
            y: rand::rng().random_range(-5..0),
        },
        body_o3: Offset {
            x: rand::rng().random_range(0..5),
            y: rand::rng().random_range(-5..0),
        },
        tentacle_o0: Offset {
            x: rand::rng().random_range(-5..5),
            y: rand::rng().random_range(-5..5),
        },
        tentacle_o1: Offset {
            x: rand::rng().random_range(-5..5),
            y: rand::rng().random_range(-5..5),
        },
        m_tentacle_o0: Offset {
            x: rand::rng().random_range(-5..5),
            y: rand::rng().random_range(-5..5),
        },
        m_tentacle_o1: Offset {
            x: rand::rng().random_range(-5..5),
            y: rand::rng().random_range(-5..5),
        },
        horns_o0: Offset {
            x: rand::rng().random_range(-5..5),
            y: rand::rng().random_range(-5..5),
        },
        horns_o1: Offset {
            x: rand::rng().random_range(-5..5),
            y: rand::rng().random_range(-5..5),
        },
    });
}
