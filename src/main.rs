extern crate getopts;
extern crate brainfuck_rust;
extern crate time;

use std::env;
use std::process::exit;
use getopts::Options;
use time::PreciseTime;
use brainfuck_rust::brainfuck::{process_input, process_input_file};

struct Config {
    zero_newline: bool,
    flush: bool
}

fn print_help(opts: Options) {
    print!("{}", opts.usage("Usage: brainfuck_rust FILE [options]"));
}

fn main() {
    let start = PreciseTime::now();
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("f", "file", "brainfuck program file", "PROGRAM");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("s", "flush", "immediately flush output to stdout");
    opts.optflag("n", "newline", "treat 10 (newline) as 0");

    let mut config = Config{zero_newline: false, flush: false};

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_help(opts);
            exit(0);
         }
    };
    if matches.opt_present("h") {
        print_help(opts);
        exit(0);
    }
    if matches.opt_present("s") {
        config.flush = true;
    }
    if matches.opt_present("n") {
        config.zero_newline = true;
    }
    if matches.opt_present("f") {
        match matches.opts_str(&["f".to_string()]) {
            Some(val) => match process_input_file(val) {
                Ok(_) => {
                    let end = PreciseTime::now();
                    println!("{}", start.to(end));
                },
                Err(err) => println!("{}", err.to_string())
            },
            None => ()
        }
    }
    else{
        process_input();
    }
}
