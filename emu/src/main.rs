use emu::execute;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, ErrorKind};

fn fmt16(n: &u16) -> String {
    format!(
        "{:04b}_{:04b}_{:04b}_{:04b}",
        (n >> 12) & 0b1111,
        (n >> 8) & 0b1111,
        (n >> 4) & 0b1111,
        n & 0b1111
    )
}

fn dump_mem(memory: &Vec<u16>) {
    let ins = [
        "ADD", "MUL", "MULH", "DIV", "MOD", "SRI", "OR", "XOR", "AND", "NOP", "MOV", "LD", "STO",
        "BEQ", "BLT", "HLT",
    ];
    let sri = [
        "ADDI", "SUBI", "SHL", "SHR", "ROL", "ROR", "NOT", "NEG", "IMM", "OUT",
    ];
    for (addr, instruction) in memory.iter().enumerate() {
        print!(
            "{}\t{}\t{}",
            addr,
            fmt16(instruction),
            ins[(instruction >> 12) as usize]
        );
        if instruction >> 12 == 0b0101 {
            print!(" {}", sri[((instruction >> 8) & 0b1111) as usize])
        }
        println!()
    }
}

fn dump_regs(regs: &Vec<u16>) {
    let mut tmp = [0; 4];
    for (no, reg) in regs.iter().enumerate() {
        println!(
            "{}\t{}\t{}\t{}",
            no,
            fmt16(reg),
            reg,
            if no == 0 {
                "zero"
            } else if no == 15 {
                "PC"
            } else {
                char::from_u32((no + 64) as u32)
                    .unwrap()
                    .encode_utf8(&mut tmp)
            }
        );
    }
}
fn main() {
    let file = File::open(
        env::args()
            .collect::<Vec<String>>()
            .get(1)
            .expect("Please provide a path"),
    )
    .expect("Cannot open file");

    let mut buf = BufReader::new(file);

    let mut bytes = [0; 2];
    let mut memory: Vec<u16> = Vec::new();
    loop {
        match buf.read(&mut bytes) {
            Ok(0) => break,
            Ok(n) => {
                assert_eq!(n, 2, "Incomplete word");
                memory.push(((bytes[0] as u16) << 8) + bytes[1] as u16)
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => panic!("{:?}", e),
        };
    }

    println!("## MEMORY ##");
    dump_mem(&memory);
    println!("\n## BEGIN EXECUTION ##");

    let (regs, out) = execute(&mut memory, 0);

    println!("{}", out);

    println!("## END EXECUTION ##");
    println!("\n## MEMORY ##");

    dump_mem(&memory);
    println!("## REGISTERS ##");
    dump_regs(&regs)
}
