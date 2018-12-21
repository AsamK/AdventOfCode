#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Registers {
    data: Vec<u64>,
}

impl Registers {
    pub fn new(registers: &[u64]) -> Self {
        Registers {
            data: registers.to_vec(),
        }
    }

    pub fn empty(registers_count: usize) -> Self {
        Registers {
            data: vec![0; registers_count],
        }
    }

    pub fn get(&self, i: u8) -> &u64 {
        &self.data[i as usize]
    }

    pub fn get_mut(&mut self, i: u8) -> &mut u64 {
        &mut self.data[i as usize]
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

pub const ALL_OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    input_a: u64,
    input_b: u64,
    output_register: u8,
}

impl Instruction {
    pub fn new(opcode: Opcode, input_a: u64, input_b: u64, output_register: u8) -> Self {
        Self {
            opcode,
            input_a,
            input_b,
            output_register,
        }
    }

    pub fn execute_instruction(&self, reg: &mut Registers) {
        match self.opcode {
            Opcode::Addr => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) + reg.get(self.input_b as u8);
            }
            Opcode::Addi => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) + self.input_b as u64;
            }
            Opcode::Mulr => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) * reg.get(self.input_b as u8);
            }
            Opcode::Muli => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) * self.input_b as u64;
            }
            Opcode::Banr => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) & reg.get(self.input_b as u8);
            }
            Opcode::Bani => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) & self.input_b as u64;
            }
            Opcode::Borr => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) | reg.get(self.input_b as u8);
            }
            Opcode::Bori => {
                *reg.get_mut(self.output_register) =
                    reg.get(self.input_a as u8) | self.input_b as u64;
            }
            Opcode::Setr => {
                *reg.get_mut(self.output_register) = *reg.get(self.input_a as u8);
            }
            Opcode::Seti => {
                *reg.get_mut(self.output_register) = self.input_a as u64;
            }
            Opcode::Gtir => {
                *reg.get_mut(self.output_register) =
                    if self.input_a as u64 > *reg.get(self.input_b as u8) {
                        1
                    } else {
                        0
                    };
            }
            Opcode::Gtri => {
                *reg.get_mut(self.output_register) =
                    if *reg.get(self.input_a as u8) > self.input_b as u64 {
                        1
                    } else {
                        0
                    };
            }
            Opcode::Gtrr => {
                *reg.get_mut(self.output_register) =
                    if *reg.get(self.input_a as u8) > *reg.get(self.input_b as u8) {
                        1
                    } else {
                        0
                    };
            }
            Opcode::Eqir => {
                *reg.get_mut(self.output_register) =
                    if self.input_a as u64 == *reg.get(self.input_b as u8) {
                        1
                    } else {
                        0
                    };
            }
            Opcode::Eqri => {
                *reg.get_mut(self.output_register) =
                    if *reg.get(self.input_a as u8) == self.input_b as u64 {
                        1
                    } else {
                        0
                    };
            }
            Opcode::Eqrr => {
                *reg.get_mut(self.output_register) =
                    if *reg.get(self.input_a as u8) == *reg.get(self.input_b as u8) {
                        1
                    } else {
                        0
                    };
            }
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self.opcode {
            Opcode::Addr => format!(
                "r{} = r{} + r{}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Addi => format!(
                "r{} = r{} + {}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Mulr => format!(
                "r{} = r{} * r{}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Muli => format!(
                "r{} = r{} + {}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Banr => format!(
                "r{} = r{} & r{}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Bani => format!(
                "r{} = r{} & {}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Borr => format!(
                "r{} = r{} | r{}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Bori => format!(
                "r{} = r{} | {}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Setr => format!("r{} = r{}", self.output_register, self.input_a),
            Opcode::Seti => format!("r{} = {}", self.output_register, self.input_a),
            Opcode::Gtir => format!(
                "r{} = {} > r{}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Gtri => format!(
                "r{} = r{} > {}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Gtrr => format!(
                "r{} = r{} > r{}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Eqir => format!(
                "r{} = {} == r{}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Eqri => format!(
                "r{} = r{} == {}",
                self.output_register, self.input_a, self.input_b
            ),
            Opcode::Eqrr => format!(
                "r{} = r{} == r{}",
                self.output_register, self.input_a, self.input_b
            ),
        }
    }
}
