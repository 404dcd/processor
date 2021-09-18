#![allow(arithmetic_overflow)]
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, ErrorKind};

const PC: usize = 15;

fn dump_mem(memory: &Vec<u16>) {
    let ins = [
        "ADD", "MUL", "MULH", "DIV", "MOD", "SRI", "OR", "XOR", "AND", "NOP", "MOV", "LD", "STO",
        "BEQ", "BLT", "HLT",
    ];
    let sri = [
        "ADDI", "SUBI", "SHL", "SHR", "ROL", "ROR", "NOT", "NEG", "IMM",
    ];
    for (addr, instruction) in memory.iter().enumerate() {
        print!(
            "{}\t{:#06b} {:#06b} {:#06b} {:#06b}\t{}",
            addr,
            instruction >> 12,
            (instruction & 0b111100000000) >> 8,
            (instruction & 0b11110000) >> 4,
            instruction & 0b1111,
            ins[(instruction >> 12) as usize]
        );
        if instruction >> 12 == 0b0101 {
            print!(" {}", sri[((instruction & 0b111100000000) >> 8) as usize])
        }
        println!()
    }
}

fn dump_regs(regs: &Vec<u16>) {
    for (no, reg) in regs.iter().enumerate() {
        println!(
            "{}\t{:#06b} {:#06b} {:#06b} {:#06b}\t{}\t{}",
            no,
            reg >> 12,
            (reg & 0b111100000000) >> 8,
            (reg & 0b11110000) >> 4,
            reg & 0b1111,
            reg,
            if no == 0 {
                "zero"
            } else if no == 15 {
                "PC"
            } else {
                ""
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

    dump_mem(&memory);

    let mut regs = vec![0u16; 16];
    loop {
        let mut instr = memory[regs[PC] as usize];
        regs[PC] += 1;
        let c = (instr & 0b1111) as usize;
        instr >>= 4;
        let b = (instr & 0b1111) as usize;
        instr >>= 4;
        let a = (instr & 0b1111) as usize;
        instr >>= 4;
        let opc = (instr & 0b1111) as usize;

        match opc {
            0b0000 => regs[c] = regs[a] + regs[b],
            0b0001 => regs[c] = regs[a] * regs[b],
            0b0010 => regs[c] = ((regs[a] as u32 * regs[b] as u32) >> 16) as u16,
            0b0011 => regs[c] = regs[a] / regs[b],
            0b0100 => regs[c] = regs[a] % regs[b],
            0b0101 => match a {
                0b0000 => regs[c] += b as u16,
                0b0001 => regs[c] -= b as u16,
                0b0010 => regs[c] <<= b,
                0b0011 => regs[c] >>= b,
                0b0100 => regs[c] <<= b, // TODO: rotate
                0b0101 => regs[c] >>= b, // TODO: rotate
                0b0110 => regs[c] = !(c as u16),
                0b0111 => regs[c] = !(c as u16) + 1,
                0b1000 => {
                    regs[c] = memory[regs[PC] as usize];
                    regs[PC] += 1
                }
                _ => panic!("unknown SRI opcode"),
            },
            0b0110 => regs[c] = regs[a] | regs[b],
            0b0111 => regs[c] = regs[a] ^ regs[b],
            0b1000 => regs[c] = regs[a] & regs[b],
            0b1001 => {}
            0b1010 => regs[c] = regs[b],
            0b1011 => regs[c] = memory[regs[a] as usize + b as usize],
            0b1100 => memory[regs[a] as usize + b as usize] = regs[c],
            0b1101 => {
                if regs[a] == regs[b] {
                    regs[PC] = regs[c]
                }
            }
            0b1110 => {
                if regs[a] < regs[b] {
                    regs[PC] = regs[c]
                }
            }
            0b1111 => break,
            _ => panic!("solar rays?"),
        }
        regs[0] = 0;
    }

    println!();
    println!();

    dump_mem(&memory);
    println!();
    dump_regs(&regs)
}
