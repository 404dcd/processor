use ass::assemble;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let file = File::open(
        env::args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("Please provide a path"),
    )
    .expect("no such file");
    let buf = BufReader::new(file);

    let code = assemble(buf.lines().map(|s| s.unwrap()).collect()).unwrap();

    let mut out = File::create("a.out").unwrap();
    out.write_all(&code).unwrap();
}
