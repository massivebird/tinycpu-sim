use std::thread::sleep;
use std::time::Duration;

use colored::Colorize;

#[derive(Debug, Default)]
struct TinyCPU {
    memory: [i8; 16],
    register: i8,
    bcd: i8,
    ir: i8,
    pc: u8,
}

impl TinyCPU {
    fn new() -> Self {
        Self::default()
    }

    fn load_instructions(&mut self, insts: [Inst; 16]) {
        for (idx, inst) in insts.into_iter().enumerate() {
            self.memory[idx] = inst.try_into().unwrap();
        }
    }

    fn run(&mut self, tick_delay: Duration) {
        // Report initial state. Further reporting is performed after running
        // each instruction.
        self.report();

        loop {
            sleep(tick_delay);

            self.ir = self.memory[self.pc as usize];

            let inst = Inst::from(self.ir);

            self.pc += 1;

            // 000    add n        [D0] <- [D0] + [n]
            // 001    and n        [D0] <- [D0] & [n]
            // 010    shl n        [n] <- [n] << 1
            // 011    disp n       BCD <- [n]
            // 100    load n       D0 <- [n]
            // 101    str n        [n] <- D0
            // 110    jmp n        pc <- n
            // 111    jz n         pc <- n if z == 1
            match inst {
                Inst::Add(n) => self.register = self.register.wrapping_add(n),
                Inst::And(n) => self.register &= n,
                Inst::Shl(n) => self.memory[n as usize] <<= 1,
                Inst::Disp(n) => self.bcd = self.memory[n as usize],
                Inst::Load(n) => self.register = self.memory[n as usize],
                Inst::Str(n) => self.memory[n as usize] = self.register,
                Inst::Jmp(n) => self.pc = n,
                Inst::Jz(_n) => todo!(), // is z the control code?
                Inst::None => (),
            }

            self.report();
        }
    }

    fn report(&self) {
        for (idx, value) in self.memory.iter().enumerate() {
            if idx as u8 == self.pc {
                let highlighted = format!("x{value:02}").reversed();
                print!("{highlighted} ");
            } else {
                print!("x{value:02} ");
            }

            if idx != 0 && (idx + 1) % 4 == 0 {
                println!();
            }
        }

        println!("PC: x{:02x}", self.pc);
        println!("BCD: {}", self.bcd);
        println!();
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
enum Inst {
    Add(i8),
    And(i8),
    Shl(u8),
    Disp(i8),
    Load(i8),
    Str(i8),
    Jmp(u8),
    Jz(u8),
    None,
}

impl From<i8> for Inst {
    fn from(value: i8) -> Self {
        // Ignore greatest bit [in opcode].
        let value = value & 0x7f;

        let opcode = value.div_euclid(16);
        let operand = value - 16i8 * opcode;

        match opcode {
            0 => Self::Add(operand),
            1 => Self::And(operand),
            2 => Self::Shl(operand as u8),
            3 => Self::Disp(operand),
            4 => Self::Load(operand),
            5 => Self::Str(operand),
            6 => Self::Jmp(operand as u8),
            7 => Self::Jz(operand as u8),
            _ => unreachable!(),
        }
    }
}

impl TryInto<i8> for Inst {
    type Error = ();

    fn try_into(self) -> Result<i8, Self::Error> {
        match self {
            Self::Add(n) => Ok(n),
            Self::And(n) => Ok(n + 16),
            Self::Shl(n) => Ok(i8::try_from(n + 16 * 2).unwrap()),
            Self::Disp(n) => Ok(n + 16 * 3),
            Self::Load(n) => Ok(n + 16 * 4),
            Self::Str(n) => Ok(n + 16 * 5),
            Self::Jmp(n) => Ok(i8::try_from(n + 16 * 6).unwrap()),
            Self::Jz(n) => Ok(i8::try_from(n + 16 * 7).unwrap()),
            Self::None => Ok(0),
        }
    }
}

fn main() {
    let insts: [Inst; 16] = [
        Inst::Add(15),
        Inst::Disp(0),
        Inst::Jmp(0),
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
        Inst::None,
    ];

    let mut c = TinyCPU::new();
    c.load_instructions(insts);
    c.run(Duration::from_millis(500));
}
