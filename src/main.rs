#[derive(Debug, Default)]
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

    fn load_instructions(&mut self, insts: [Inst; 16]) {
        for (idx, inst) in insts.into_iter().enumerate() {
            self.memory[idx] = inst.try_into().unwrap();
        }
    }

    fn run(&mut self) {
        for val in self.memory {
            let Ok(inst) = Inst::try_from(val) else {
                continue;
            };

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
                Inst::Shl(n) => self.memory[n as usize] <<= 1,
                Inst::Disp(n) => self.bcd = self.memory[n as usize],
                Inst::Load(n) => self.register = self.memory[n as usize],
                Inst::Str(n) => self.memory[n as usize] = self.register,
                Inst::Jmp(n) => self.pc = n,
                Inst::Jz(_n) => todo!(), // is z the control code?
                Inst::None => (),
            }

            self.pc += 1;
        }
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

impl TryFrom<i8> for Inst {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 16 {
            return Ok(Self::Add(value));
        }

        match value.div_euclid(16) {
            1 => Ok(Self::And(value)),
            2 => Ok(Self::Shl(value as u8)),
            3 => Ok(Self::Disp(value)),
            4 => Ok(Self::Str(value)),
            5 => Ok(Self::Jmp(value as u8)),
            6 => Ok(Self::Jz(value as u8)),
            _ => Err(()),
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
        Inst::None,
        Inst::None,
    ];

    let mut c = Computer::new();
    c.load_instructions(insts);
    c.run();

    dbg!(c);
}
