use crate::Register;
use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt};

/// The table that is used to lookup the format of a given opcode.
///
/// 1: R format
/// 2: I format
/// 3: J format
const FORMAT_TABLE: Lazy<HashMap<u8, u8>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0b000000, 1);
    map.insert(0b001000, 2);
    map.insert(0b001001, 2);
    map.insert(0b001100, 2);
    map.insert(0b000100, 2);
    map.insert(0b000001, 2);
    map.insert(0b000111, 2);
    map.insert(0b000110, 2);
    map.insert(0b000101, 2);
    map.insert(0b000010, 3);
    map.insert(0b000011, 3);
    map.insert(0b100000, 2);
    map.insert(0b001111, 2);
    map.insert(0b100011, 2);
    map.insert(0b001101, 2);
    map.insert(0b101000, 2);
    map.insert(0b001010, 2);
    map.insert(0b001011, 2);
    map.insert(0b101011, 2);
    map.insert(0b001110, 2);
    map
});

pub fn parse(raw: u32) -> Option<Instruction> {
    let opcode = ((raw >> 25) & 0x3F) as u8;
    let format = FORMAT_TABLE.get(&opcode)?.clone();
    match format {
        // The R format
        // opcode(6) rs(5) rt(5) rd(5) shamt(5) funct(6)
        1 => {
            let rs = ((raw >> 21) & 0x1F) as Register;
            let rt = ((raw >> 16) & 0x1F) as Register;
            let rd = ((raw >> 11) & 0x1F) as Register;
            let shamt = ((raw >> 6) & 0x1F) as u8;
            let funct = (raw & 0x3F) as u8;

            let kind = match funct {
                0b000000 if shamt == 0 => Kind::Noop,
                0b100000 => Kind::Add,
                0b100001 => Kind::Addu,
                0b100100 => Kind::And,
                0b011010 => Kind::Div,
                0b011011 => Kind::Divu,
                0b001000 => Kind::Jr,
                0b010000 => Kind::Mfhi,
                0b010010 => Kind::Mflo,
                0b011000 => Kind::Mult,
                0b011001 => Kind::Multu,
                0b100101 => Kind::Or,
                0b000000 => Kind::Sll,
                0b000100 => Kind::Sllv,
                0b101010 => Kind::Slt,
                0b101011 => Kind::Sltu,
                0b000011 => Kind::Sra,
                0b000010 => Kind::Srl,
                0b000110 => Kind::Srlv,
                0b100010 => Kind::Sub,
                0b100011 => Kind::Subu,
                0b100110 => Kind::Xor,

                0b001100 => Kind::Syscall,
                _ => return None,
            };
            let format = Format::R {
                rs,
                rt,
                rd,
                shamt,
                funct,
            };

            Some(Instruction { raw, kind, format })
        }
        // The I format
        // opcode(6) rs(5) rt(5) immediate(16)
        2 => {
            let rs = ((raw >> 21) & 0x1F) as Register;
            let rt = ((raw >> 16) & 0x1F) as Register;
            let val = (raw & 0xFFFF) as u16;

            let kind = match (opcode, rt) {
                (0b001000, _) => Kind::Addi,
                (0b001001, _) => Kind::Addiu,
                (0b001100, _) => Kind::Andi,
                (0b000100, _) => Kind::Beq,
                (0b000001, 0b00001) => Kind::Bgez,
                (0b000001, 0b10001) => Kind::Bgezal,
                (0b000001, 0b00000) => Kind::Bltz,
                (0b000001, 0b10000) => Kind::Bltzal,

                (0b000111, 0b00000) => Kind::Bgtz,
                (0b000110, 0b00000) => Kind::Blez,
                (0b000101, _) => Kind::Bne,

                (0b100000, _) => Kind::Lb,
                (0b001111, _) => Kind::Lui,
                (0b100011, _) => Kind::Lw,
                (0b001101, _) => Kind::Ori,
                (0b101000, _) => Kind::Sb,
                (0b001010, _) => Kind::Slti,
                (0b001011, _) => Kind::Sltiu,
                (0b101011, _) => Kind::Sw,
                (0b001110, _) => Kind::Xori,
                _ => return None,
            };
            let format = Format::I { rs, rt, val };

            Some(Instruction { raw, format, kind })
        }
        // The J format
        // opcode(6) address(26)
        3 => {
            let val = raw & 0x3FFFFFF;

            Some(Instruction {
                raw,
                format: Format::J(val),
                kind: match opcode {
                    0b000010 => Kind::J,
                    0b000011 => Kind::Jal,
                    _ => unreachable!(),
                },
            })
        }
        _ => unreachable!(),
    }
}

#[test]
fn test() {
    let raw = 0x00220821;
    if let Some(result) = parse(raw) {
        println!("{}", result);
    } else {
        println!("None");
    }
}

#[derive(Debug)]
pub struct Instruction {
    raw: u32,
    kind: Kind,
    format: Format,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.format)
    }
}

#[derive(Debug, Clone)]
pub enum Format {
    R {
        rs: Register,
        rt: Register,
        rd: Register,
        shamt: u8,
        funct: u8,
    },
    I {
        rs: Register,
        rt: Register,
        val: u16,
    },
    J(u32),
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::R { rs, rt, rd, .. } => write!(f, "${} ${} ${}", rs, rt, rd),
            Format::I { rs, rt, val } => write!(f, "${} ${} {:x}", rs, rt, val),
            Format::J(addr) => write!(f, "{:x}", addr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Kind {
    Add,
    Addi,
    Addiu,
    Addu,
    And,
    Andi,
    Beq,
    Bgez,
    Bgezal,
    Bgtz,
    Blez,
    Bltz,
    Bltzal,
    Bne,
    Div,
    Divu,
    J,
    Jal,
    Jr,
    Lb,
    Lui,
    Lw,
    Mfhi,
    Mflo,
    Mult,
    Multu,
    Noop,
    Or,
    Ori,
    Sb,
    Sll,
    Sllv,
    Slt,
    Slti,
    Sltiu,
    Sltu,
    Sra,
    Srl,
    Srlv,
    Sub,
    Subu,
    Sw,
    Syscall,
    Xor,
    Xori,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Kind::Add => "add",
            Kind::Addi => "addi",
            Kind::Addiu => "addiu",
            Kind::Addu => "addu",
            Kind::And => "and",
            Kind::Andi => "andi",
            Kind::Beq => "beq",
            Kind::Bgez => "bgez",
            Kind::Bgezal => "bgezal",
            Kind::Bgtz => "bgtz",
            Kind::Blez => "blez",
            Kind::Bltz => "bltz",
            Kind::Bltzal => "bltzal",
            Kind::Bne => "bne",
            Kind::Div => "div",
            Kind::Divu => "divu",
            Kind::J => "j",
            Kind::Jal => "jal",
            Kind::Jr => "jr",
            Kind::Lb => "lb",
            Kind::Lui => "lui",
            Kind::Lw => "lw",
            Kind::Mfhi => "mfhi",
            Kind::Mflo => "mflo",
            Kind::Mult => "mult",
            Kind::Multu => "multu",
            Kind::Noop => "noop",
            Kind::Or => "or",
            Kind::Ori => "ori",
            Kind::Sb => "sb",
            Kind::Sll => "sll",
            Kind::Sllv => "sllv",
            Kind::Slt => "slt",
            Kind::Slti => "slti",
            Kind::Sltiu => "sltiu",
            Kind::Sltu => "sltu",
            Kind::Sra => "sra",
            Kind::Srl => "srl",
            Kind::Srlv => "srlv",
            Kind::Sub => "sub",
            Kind::Subu => "subu",
            Kind::Sw => "sw",
            Kind::Syscall => "syscall",
            Kind::Xor => "xor",
            Kind::Xori => "xori",
        };
        write!(f, "{}", repr)
    }
}
