use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let urladdr = std::env::args().nth(1).expect("no pattern given");
    let mut easy = Easy::new();
    easy.url(&urladdr).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();

    println!("Get Request on address: {urladdr}");
    println!("{}", easy.response_code().unwrap());
}
