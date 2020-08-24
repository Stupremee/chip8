use crate::Register;
use std::fmt;

pub fn parse(raw: u32) -> Option<Instruction> {}

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
            Format::R {
                rs,
                rt,
                rd,
                shamt,
                funct,
            } => write!(f, "${} ${} ${} {} {:x}", rs, rt, rd, shamt, funct),
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
