use std::io::prelude::*;
use std::io::BufReader;
use std::env;

extern crate chrono;
use chrono::prelude::*;

fn usage() {
    println!("Usage: outtime [-f <strftime_format>]");
    println!("    Prefixes each line of stdout with the timestamp.");
    println!("    Use -f to customize the timestamp.");
    println!("    See https://docs.rs/chrono/latest/chrono/format/strftime/index.html");
    std::process::exit(0);
}

fn main() -> std::io::Result<()> {
    let mut format = "%Y/%m/%d:%T%.3f";

    let args: Vec<String> = env::args().collect();
    for i in 0 .. args.len() {
        match args[i].as_str() {
            "-f" => {
                if i < args.len() - 1 {
                    format = &args[i + 1];
                } else {
                    usage();
                }
            },
            "-?" => {
                usage();
            },
            _ => ()
        }
    }

    for line in BufReader::new(std::io::stdin()).lines() {
        let now = Local::now();
        println!("{} {}", now.format(format), line.unwrap());
    }

    Ok(())
}
