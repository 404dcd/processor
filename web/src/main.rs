#[macro_use]
extern crate rocket;
use ass::assemble;
use emu::execute;
use rocket::fs::{relative, FileServer};

#[post("/", data = "<program>")]
fn run(program: &str) -> String {
    let codeu8s = match assemble(program.lines().map(|s| s.to_owned()).collect()) {
        Ok(x) => x,
        Err(y) => return y,
    };
    let mut codeu16s: Vec<u16> = Vec::new();

    for ind in 0..codeu8s.len() {
        if ind % 2 == 0 {
            codeu16s.push((codeu8s[ind] as u16) << 8)
        } else {
            codeu16s[ind / 2] += codeu8s[ind] as u16
        }
    }

    match execute(&mut codeu16s, 1_000_000) {
        Ok(x) => {
            if x.1.len() > 10_000 {
                let mut ret = x.1[..10_000].to_string();
                ret.push_str("\n## Output trimmed");
                ret
            } else {
                x.1
            }
        }
        Err(y) => y,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", routes![run])
}
