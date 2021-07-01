#![allow(dead_code)]
#![allow(unused)]
#![allow(non_snake_case)]

mod data;
mod mach;
mod types;

use clap::{App, Arg};
use std::fs;

use types::Data::Int;
use types::{Mach, PCode, Data};
// (setq lsp-eldoc-hook nil)

fn main() {
    let matches = App::new("ruforth")
        .version("0.1.0")
        .author("Derek Rhodes <physci@gmail.com>")
        .about("A rust implmentation of forth")
        .arg(
            Arg::with_name("infile")
                .short("i")
                .long("infile")
                .takes_value(true)
                .help("An input file containing forth code"),
        )
        .get_matches();

    let mut m = Mach::new();

    match matches.value_of("infile") {
        Some(fname) => {
            let contents =
                fs::read_to_string(fname).expect(&("couldn't read file: ".to_owned() + fname));
            m.initCode = contents;
        }
        None => {}
    }

    crate::mach::main_loop(m.clone());
    println!("{:?}", "done");
}
