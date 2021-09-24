#![allow(arithmetic_overflow)]

const PC: usize = 15;

pub fn execute(memory: &mut Vec<u16>, lim: u64) -> (Vec<u16>, String) {
    let mut out: String = "".to_owned();
    let mut regs = vec![0u16; 16];
    let mut count: u64 = 0;
    loop {
        let mut instr = memory[regs[PC] as usize];
        count += 1;
        if count > lim && lim != 0 {
            out.push_str("## Timeout");
            break;
        }
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
                0b0100 => regs[c] = regs[c].rotate_left(b as u32),
                0b0101 => regs[c] = regs[c].rotate_right(b as u32),
                0b0110 => regs[c] = !(c as u16),
                0b0111 => regs[c] = !(c as u16) + 1,
                0b1000 => {
                    let read = memory[regs[PC] as usize];
                    regs[PC] += 1;
                    regs[c] = read;
                }
                0b1001 => out.push_str(&format!("{}\n", regs[c])),
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
    (regs, out)
}
