extern crate bs58;

use std::io::{ self, Read, Write };

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    match bs58::decode(input.trim()) {
        Ok(vec) => io::stdout().write_all(&*vec).unwrap(),
        Err(err) => writeln!(io::stderr(), "{}", err).unwrap(),
    };
}
