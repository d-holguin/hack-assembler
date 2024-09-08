use crate::AsmError;

#[derive(Debug, Clone)]
pub enum Instruction {
    A(AInstruction),
    C(CInstruction),
    L(String),
    Variable(String),
}
impl Instruction {
    pub fn parse<S: AsRef<str>>(
        instruction: S,
        line_number: usize,
    ) -> Result<Instruction, AsmError> {
        let instruction = instruction.as_ref().trim();

        if instruction.starts_with('@') {
            let symbol = &instruction[1..];
            if symbol.is_empty() {
                return Err(AsmError::InvalidInstruction {
                    line: line_number,
                    instruction: instruction.to_string(),
                });
            }

            if let Ok(num) = symbol.parse::<u16>() {
                // Directly addressable number
                Ok(Instruction::A(AInstruction::new(num)))
            } else {
                // Variable
                Ok(Instruction::Variable(symbol.to_string()))
            }
        } else if instruction.starts_with('(') && instruction.ends_with(')') {
            let label = instruction.trim_matches(|c: char| c == '(' || c == ')');
            return Ok(Instruction::L(label.to_string()));
        } else {
            // Assuming the rest are C-instructions
            return parse_c_instruction(instruction, line_number);
        }
    }
}

impl CInstruction {
    pub fn to_binary(&self) -> String {
        let a_bit = if self.comp.clone().uses_m() { "1" } else { "0" };
        let dest = self.dest.clone() as u16;
        let comp = self.comp.clone().to_binary();
        let jump = self.jump.clone() as u16;
        format!("111{}{:06b}{:03b}{:03b}", a_bit, comp, dest, jump)
    }
}

fn parse_c_instruction(instruction: &str, line_number: usize) -> Result<Instruction, AsmError> {
    let parts: Vec<&str> = instruction.split(';').collect();
    let comp_dest = parts[0];
    let jump = parts.get(1).unwrap_or(&"null");

    let comp_dest_parts: Vec<&str> = comp_dest.split('=').collect();
    let (dest, comp) = if comp_dest_parts.len() == 2 {
        (comp_dest_parts[0], comp_dest_parts[1])
    } else if comp_dest_parts.len() == 1 {
        ("null", comp_dest_parts[0])
    } else {
        return Err(AsmError::InvalidInstruction {
            line: line_number,
            instruction: instruction.to_string(),
        });
    };

    let dest_enum = Dest::try_from((dest, line_number)).unwrap_or(Dest::Null);
    let comp_enum = Comp::try_from((comp, line_number))?;
    let jump_enum = Jump::try_from((*jump, line_number)).unwrap_or(Jump::Null);

    Ok(Instruction::C(CInstruction::new(
        dest_enum, comp_enum, jump_enum,
    )))
}

#[derive(Debug, Clone)]
pub struct AInstruction {
    value: u16,
}

#[derive(Debug, Clone)]
pub struct CInstruction {
    dest: Dest,
    comp: Comp,
    jump: Jump,
}

impl AInstruction {
    pub fn new(value: u16) -> Self {
        AInstruction { value }
    }

    pub fn value(&self) -> u16 {
        self.value
    }
}

impl CInstruction {
    pub fn new(dest: Dest, comp: Comp, jump: Jump) -> Self {
        CInstruction { dest, comp, jump }
    }
}

#[derive(Debug, Clone)]
pub enum Dest {
    Null = 0b000,
    M = 0b001,
    D = 0b010,
    MD = 0b011,
    A = 0b100,
    AM = 0b101,
    AD = 0b110,
    AMD = 0b111,
}

impl TryFrom<(&str, usize)> for Dest {
    type Error = AsmError;

    fn try_from(value: (&str, usize)) -> Result<Self, Self::Error> {
        let (input, line) = value;
        match input {
            "null" => Ok(Dest::Null),
            "M" => Ok(Dest::M),
            "D" => Ok(Dest::D),
            "MD" => Ok(Dest::MD),
            "A" => Ok(Dest::A),
            "AM" => Ok(Dest::AM),
            "AD" => Ok(Dest::AD),
            "AMD" => Ok(Dest::AMD),
            _ => Err(AsmError::InvalidInstruction {
                line,
                instruction: input.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Comp {
    Zero = 0b0101010,
    One = 0b0111111,
    NegOne = 0b0111010,
    D = 0b0001100,
    A = 0b0110000,
    NotD = 0b0001101,
    NotA = 0b0110001,
    NegD = 0b0001111,
    NegA = 0b0110011,
    DPlusOne = 0b0011111,
    APlusOne = 0b0110111,
    DMinusOne = 0b0001110,
    AMinusOne = 0b0110010,
    DPlusA = 0b0000010,
    DMinusA = 0b0010011,
    AMinusD = 0b0000111,
    DAndA = 0b0000000,
    DOrA = 0b0010101,
    M = 0b1110000,
    NotM = 0b1110001,
    NegM = 0b1110011,
    MPlusOne = 0b1110111,
    MMinusOne = 0b1110010,
    DPlusM = 0b1000010,
    DMinusM = 0b1010011,
    MMinusD = 0b1000111,
    DAndM = 0b1000000,
    DOrM = 0b1010101,
}

impl Comp {
    pub fn to_binary(self) -> u16 {
        self as u16 & 0b0111111 // Strip the 'a-bit', return only the 6-bit computation code
    }
    pub fn uses_m(self) -> bool {
        self as u16 & 0b1000000 != 0
    }
}

impl TryFrom<(&str, usize)> for Comp {
    type Error = AsmError;

    fn try_from(value: (&str, usize)) -> Result<Self, Self::Error> {
        let (input, line) = value;
        match input {
            "0" => Ok(Comp::Zero),
            "1" => Ok(Comp::One),
            "-1" => Ok(Comp::NegOne),
            "D" => Ok(Comp::D),
            "A" => Ok(Comp::A),
            "!D" => Ok(Comp::NotD),
            "!A" => Ok(Comp::NotA),
            "-D" => Ok(Comp::NegD),
            "-A" => Ok(Comp::NegA),
            "D+1" => Ok(Comp::DPlusOne),
            "A+1" => Ok(Comp::APlusOne),
            "D-1" => Ok(Comp::DMinusOne),
            "A-1" => Ok(Comp::AMinusOne),
            "D+A" => Ok(Comp::DPlusA),
            "D-A" => Ok(Comp::DMinusA),
            "A-D" => Ok(Comp::AMinusD),
            "D&A" => Ok(Comp::DAndA),
            "D|A" => Ok(Comp::DOrA),
            "M" => Ok(Comp::M),
            "!M" => Ok(Comp::NotM),
            "-M" => Ok(Comp::NegM),
            "M+1" => Ok(Comp::MPlusOne),
            "M-1" => Ok(Comp::MMinusOne),
            "D+M" => Ok(Comp::DPlusM),
            "D-M" => Ok(Comp::DMinusM),
            "M-D" => Ok(Comp::MMinusD),
            "D&M" => Ok(Comp::DAndM),
            "D|M" => Ok(Comp::DOrM),
            _ => Err(AsmError::InvalidInstruction {
                line,
                instruction: input.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Jump {
    Null = 0b000,
    JGT = 0b001,
    JEQ = 0b010,
    JGE = 0b011,
    JLT = 0b100,
    JNE = 0b101,
    JLE = 0b110,
    JMP = 0b111,
}

impl TryFrom<(&str, usize)> for Jump {
    type Error = AsmError;

    fn try_from(value: (&str, usize)) -> Result<Self, Self::Error> {
        let (input, line) = value;
        match input {
            "null" => Ok(Jump::Null),
            "JGT" => Ok(Jump::JGT),
            "JEQ" => Ok(Jump::JEQ),
            "JGE" => Ok(Jump::JGE),
            "JLT" => Ok(Jump::JLT),
            "JNE" => Ok(Jump::JNE),
            "JLE" => Ok(Jump::JLE),
            "JMP" => Ok(Jump::JMP),
            _ => Err(AsmError::InvalidInstruction {
                line,
                instruction: input.to_string(),
            }),
        }
    }
}
