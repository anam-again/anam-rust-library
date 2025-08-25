#![allow(dead_code)]

use std::{env, process};

use hello_rust::{
    cli_builder::{Command, Parameter},
    grid_printer,
};

mod cli_builder;
mod ip_sniffer;

fn main() {
    let rows: &mut[u32; 32] = &mut[0; 32];
    rows[5] = 10;
    rows[6] = 14;
    grid_printer::set_px(rows, (6, 1), true);
    grid_printer::print_array(rows);
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
    match command.process_args(&args) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            process::exit(1);
        }
    };
    grid_printer::print_line(0xFFFFFFFF);
}
