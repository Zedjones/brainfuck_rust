extern crate getopts;
extern crate brainfuck_rust;

use std::env;
use std::process::exit;
use getopts::Options;
use brainfuck_rust::brainfuck::process_input;

struct Config {
    zero_newline: bool,
    flush: bool
}

fn print_help(opts: Options) {
    print!("{}", opts.usage("Usage: brainfuck_rust FILE [options]"));
}

fn main() {
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
        print_help(opts)
    }
    if matches.opt_present("s") {
        config.flush = true;
    }
    if matches.opt_present("n") {
        config.zero_newline = true;
    }
    if matches.opt_present("f") {
        match matches.opts_str(&["f".to_string()]) {
            Some(val) => println!("{}", val),
            None => ()
        }
    }
    process_input();
}
