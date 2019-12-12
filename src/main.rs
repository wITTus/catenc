extern crate base64;
extern crate clap;
extern crate catenc;

use std::io::{self, BufRead};
use std::io::Write;

use clap::{App, Arg, values_t};
use catenc::lib::{process_lines_tokenized, process_lines};

fn main() {
    let matches = App::new("catenc")
        .version("1.0")
        .author("Sascha W.")
        .about("Category/Label encoder for the shell. Converts categorical columns to numeric columns.")
        .arg(Arg::with_name("numeric").short("n").help("Numeric text output (default)"))
        .arg(Arg::with_name("base64").short("e").help("Base64 text output"))
        .arg(Arg::with_name("separator").short("t").takes_value(true).help("Column separator (default: ' ')"))
        .arg(Arg::with_name("indices").short("k").require_delimiter(true).help("Column index, starting at 1"))
        .get_matches();

    // Stdout
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let print_fn = |line| { writeln!(out, "{}", line).expect("Couldn't write to stdout!"); };

    // Stdin
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.expect("Couldn't read line!"));

    // Call
    let mode_base64 = matches.is_present("base64");

    if matches.is_present("indices") {
        let indices = values_t!(matches, "indices", usize).unwrap_or_default();
        let separator = matches.value_of("separator").unwrap_or(" ");

        process_lines_tokenized(lines, mode_base64, separator, indices, print_fn);
    } else {
        process_lines(lines, mode_base64, print_fn);
    }
}