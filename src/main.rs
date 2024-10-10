#[derive(Default)]
struct Computer {
    memory: [i8; 16],
    register: i8,
    bcd: i8,
    pc: u8,
}

impl Computer {
    fn new() -> Self {
        Self::default()
    }

    fn execute_instructions(&mut self, insts: &[Inst]) {
        for inst in insts {
            // 000    add n        [D0] <- [D0] + [n]
            // 001    and n        [D0] <- [D0] & [n]
            // 010    shl n        [n] <- [n] << 1
            // 011    disp n       BCD <- [n]
            // 100    load n       D0 <- [n]
            // 101    str n        [n] <- D0
            // 110    jmp n        pc <- n
            // 111    jz n         pc <- n if z == 1
            match inst {
                Inst::Add(n) => self.register += n,
                Inst::And(n) => self.register &= n,
                Inst::Shl(n) => self.memory[*n as usize] <<= 1,
                Inst::Disp(n) => self.bcd = self.memory[*n as usize],
                Inst::Load(n) => self.register = self.memory[*n as usize],
                Inst::Str(n) => self.memory[*n as usize] = self.register,
                Inst::Jmp(n) => self.pc = *n,
                Inst::Jz(n) => todo!(), // is z the control code?
            }
        }
    }
}

enum Inst {
    Add(i8),
    And(i8),
    Shl(u8),
    Disp(i8),
    Load(i8),
    Str(i8),
    Jmp(u8),
    Jz(u8),
}

fn main() {
    println!("Hello, world!");
}
