#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
    )]

extern crate ansi_term;
extern crate hyper;

use hyper::Client;
use std::env;
use std::io::Read;
use ansi_term::Colour::{Cyan, Green, Red, White};

#[cfg(target_os = "macos")]
const PLATFORM: &'static str = "osx";
#[cfg(not(target_os = "macos"))]
const PLATFORM: &'static str = "linux";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let error = "Specify an argument!";
        println!("{}", format!("{error} \nExample: \n$ tldr ls", error = Red.paint(error).to_string()));
        process::exit(1);
    }

    let common_url = format!("https://raw.github.com/tldr-pages/tldr/master/pages/common/{page}.md", page = args[1]);
    let platform_url = format!("https://raw.github.com/tldr-pages/tldr/master/pages/{platform}/{page}.md", page = args[1], platform=PLATFORM);

    let client = Client::new();

    // Creating an outgoing request.
    let mut res = match client.get(&common_url).send() {
        Ok(res) => res,
        _ => client.get(&platform_url).send().unwrap(),
    };

    // Read the Response.
    let mut the_tldr = String::new();
    res.read_to_string(&mut the_tldr).unwrap();

    render(&the_tldr);
}

fn render(the_tldr: &str) {    
    for line in the_tldr.split("\n") {
        let rendered = match line.chars().next() {

            // Heading
            Some('#') => Cyan.paint(line).to_string(),

            // Quotation
            Some('>') => White.paint(line).to_string(),

            // Inline list
            Some('-') => Green.paint(line).to_string(),

            // Code
            Some('`') => White.paint(line).to_string(),

            _   => "".to_string()
        };

        println!("{}", rendered);
    }   
}