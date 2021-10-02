use std::collections::HashMap;

fn prs_string(arg: &str) -> Result<u16, String> {
    Ok(if arg.starts_with("0b") {
        u16::from_str_radix(arg.trim_start_matches("0b"), 2)
    } else if arg.starts_with("0x") {
        u16::from_str_radix(arg.trim_start_matches("0x"), 16)
    } else {
        arg.parse()
    }
    .map_err(|e| format!("{}: '{}' when converting number", e, arg))?)
}

fn arg_to_bin(arg: &str) -> Result<u8, String> {
    if arg.starts_with("$") {
        return match arg.trim_start_matches("$") {
            "0" => Ok(0),
            "pc" => Ok(15),
            chrs => {
                let ret = chrs.bytes().next().ok_or_else(|| "failure")? - 96;
                if ret >= 0b0001 && ret <= 0b1101 {
                    Ok(ret)
                } else {
                    Err(format!("could not address register {}", arg))
                }
            }
        };
    }

    let ret = prs_string(arg)?;

    if !(ret <= 0b1111) {
        return Err(format!("could not use arg '{}' as 4 bits", ret));
    }
    Ok(ret as u8)
}

pub fn assemble(program: Vec<String>) -> Result<Vec<u8>, String> {
    let mut offset = 0u16;
    let mut labels: HashMap<String, u16> = HashMap::new();

    let mut nlines: Vec<String> = Vec::new();
    let doubles = ["imm", "jmp"];
    let triples = ["beq", "blt"];
    for line in program {
        // First pass, label offsets
        let line = line
            .find(";")
            .map(|idx| &line[..idx])
            .unwrap_or(&line)
            .trim();

        if line.len() < 2 {
            continue; // shortest instruction is 2 chars
        }

        if line.ends_with(":") && line.starts_with(".") {
            let lbl = line.trim_end_matches(":").trim_start_matches(".").trim();
            let test = lbl.chars().next();
            if test.ok_or_else(|| "empty label")?.is_numeric() {
                return Err(format!("label '{}' cannot be number", lbl));
            }
            labels.insert(lbl.to_owned(), offset);
        } else {
            nlines.push(line.to_owned());
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
        ("out", 0b0101_1001),
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
        let mut sline = line.split_ascii_whitespace();
        let instr = sline.next().ok_or_else(|| "failed to iter next line")?;
        let mut args: Vec<&str> = sline.collect();
        assert!(args.len() <= 3);

        if opcodes.contains_key(instr) {
            let mut c = *opcodes.get(instr).ok_or_else(|| "lookup failure")?;
            if args.len() == 3 {
                c += arg_to_bin(args[0])?;
                args = args[1..].to_vec()
            }
            code.push(c);

            c = 0;
            if args.len() == 2 {
                c += arg_to_bin(args[0])? << 4;
                args = args[1..].to_vec()
            }

            if args.len() == 1 {
                c += arg_to_bin(args[0])?
            }
            code.push(c);
        } else {
            match instr {
                "imm" => {
                    code.push(0b0101_1000);
                    code.push(arg_to_bin(args[1])?);
                    let data: u16;
                    if let Some(x) = labels.get(args[0]) {
                        data = *x
                    } else {
                        data = prs_string(args[0])?
                    }
                    code.push((data >> 8) as u8);
                    code.push((data & 0xff) as u8);
                }
                "jmp" => {
                    code.push(0b0101_1000);
                    code.push(0b0000_1111);
                    let data = labels
                        .get(args[0])
                        .ok_or_else(|| format!("undefined label referenced: '{}'", line))?;
                    code.push((data >> 8) as u8);
                    code.push((data & 0xff) as u8);
                }
                "beq" => {
                    code.push(0b0101_1000);
                    code.push(0b0000_1110); // temp reg 14
                    let data = labels
                        .get(args[2])
                        .ok_or_else(|| format!("undefined label referenced: '{}'", line))?;
                    code.push((data >> 8) as u8); // imm label address
                    code.push((data & 0xff) as u8);
                    code.push((0b1101 << 4) + arg_to_bin(args[0])?);
                    code.push((arg_to_bin(args[1])? << 4) + 0b1110);
                }
                "blt" => {
                    code.push(0b0101_1000);
                    code.push(0b0000_1110); // temp reg 14
                    let data = labels
                        .get(args[2])
                        .ok_or_else(|| format!("undefined label referenced: '{}'", line))?;
                    code.push((data >> 8) as u8); // imm label address
                    code.push((data & 0xff) as u8);
                    code.push((0b1110 << 4) + arg_to_bin(args[0])?);
                    code.push((arg_to_bin(args[1])? << 4) + 0b1110); // temp reg 14
                }
                _ => return Err(format!("unrecognised instruction {}", instr)),
            }
        }
    }
    Ok(code)
}
