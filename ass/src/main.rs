use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn arg_to_bin(arg: &str) -> u8 {
    if arg == "$0" {
        return 0;
    }
    if arg == "$pc" {
        return 15;
    }
    if arg.starts_with("$") {
        let ret: u8 = arg.bytes().last().unwrap() - 96;
        assert!(ret >= 0b0001 && ret <= 0b1110);
        return ret;
    }
    let ret: u8 = arg.parse().unwrap();
    assert!(ret <= 0b1111);
    ret
}

fn main() {
    let file = File::open(
        env::args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("Please provide a path"),
    )
    .expect("no such file");
    let buf = BufReader::new(file);

    let mut offset = 0u16;
    let mut labels: HashMap<String, u16> = HashMap::new();

    let mut nlines: Vec<String> = Vec::new();
    let doubles = ["imm", "jmp"];
    let triples = ["beq", "blt"];
    for line in buf.lines() {
        // First pass, label offsets
        let line = line.unwrap();
        let line = line
            .find(";")
            .map(|idx| &line[..idx])
            .unwrap_or(&line)
            .trim();

        if line.len() < 2 {
            continue; // shortest instruction is 2 chars
        }

        nlines.push(line.to_owned());
        if line.ends_with(":") {
            labels.insert(line.trim_end_matches(";").to_owned(), offset);
        } else {
            offset += 1;
            for double in doubles {
                if line.starts_with(double) {
                    offset += 1;
                }
            }
            for triple in triples {
                if line.starts_with(triple) {
                    offset += 2;
                }
            }
        }
    }

    let opcodes: HashMap<&str, u8> = [
        ("add", 0b0000 << 4),
        ("mul", 0b0001 << 4),
        ("mulh", 0b0010 << 4),
        ("div", 0b0011 << 4),
        ("mod", 0b0100 << 4),
        ("addi", 0b0101_0000),
        ("subi", 0b0101_0001),
        ("shl", 0b0101_0010),
        ("shr", 0b0101_0011),
        ("rol", 0b0101_0100),
        ("ror", 0b0101_0101),
        ("not", 0b0101_0110),
        ("neg", 0b0101_0111),
        ("or", 0b0110 << 4),
        ("xor", 0b0111 << 4),
        ("and", 0b1000 << 4),
        ("nop", 0b1001 << 4),
        ("mov", 0b1010 << 4),
        ("ld", 0b1011 << 4),
        ("sto", 0b1100 << 4),
        ("hlt", 0b1111 << 4),
    ]
    .iter()
    .cloned()
    .collect();

    let mut code: Vec<u8> = Vec::new();
    for line in nlines {
        let mut line = line.split_ascii_whitespace();
        let instr = line.next().unwrap();
        let mut args: Vec<&str> = line.collect();
        assert!(args.len() <= 3);

        if opcodes.contains_key(instr) {
            let mut c = *opcodes.get(instr).unwrap();
            if args.len() == 3 {
                c += arg_to_bin(args[0]);
                args = args[1..].to_vec()
            }
            code.push(c);

            c = 0;
            if args.len() == 2 {
                c += arg_to_bin(args[0]) << 4;
                args = args[1..].to_vec()
            }

            if args.len() == 1 {
                c += arg_to_bin(args[0])
            }
            code.push(c);
        } else {
            match instr {
                "imm" => {
                    code.push(0b0101_1000);
                    code.push(arg_to_bin(args[1]));
                    let data = args[0].parse::<u16>().unwrap();
                    code.push((data & 0xff00) as u8);
                    code.push((data & 0xff) as u8);
                }
                "jmp" => {
                    code.push(0b0101_1000);
                    code.push(0b0000_1111);
                    let data = labels.get(args[0]).unwrap();
                    code.push((data & 0xff00) as u8);
                    code.push((data & 0xff) as u8);
                }
                "beq" => {
                    code.push(0b0101_1000);
                    code.push(0b0000_1110); // temp reg 14
                    let data = labels.get(args[2]).unwrap();
                    code.push((data & 0xff00) as u8); // imm label address
                    code.push((data & 0xff) as u8);
                    code.push((0b1101 << 4) + arg_to_bin(args[0]));
                    code.push((arg_to_bin(args[1]) << 4) + 0b1110);
                }
                "blt" => {
                    code.push(0b0101_1000);
                    code.push(0b0000_1110); // temp reg 14
                    let data = labels.get(args[2]).unwrap();
                    code.push((data & 0xff00) as u8); // imm label address
                    code.push((data & 0xff) as u8);
                    code.push((0b1110 << 4) + arg_to_bin(args[0]));
                    code.push((arg_to_bin(args[1]) << 4) + 0b1110); // temp reg 14
                }
                _ => panic!("unrecognised instruction"),
            }
        }
    }

    let mut out = File::create("a.out").unwrap();
    out.write_all(&code).unwrap();
}
