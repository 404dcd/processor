#[macro_use]
extern crate rocket;
use ass::assemble;
use emu::execute;
use rocket::fs::{relative, FileServer};

#[post("/", data = "<program>")]
fn run(program: &str) -> String {
    let codeu8s = assemble(program.lines().map(|s| s.to_owned()).collect());
    let mut codeu16s: Vec<u16> = Vec::new();

    for ind in 0..codeu8s.len() {
        if ind % 2 == 0 {
            codeu16s.push((codeu8s[ind] as u16) << 8)
        } else {
            codeu16s[ind / 2] += codeu8s[ind] as u16
        }
    }

    execute(&mut codeu16s, 1_000_000).1
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![run])
}
