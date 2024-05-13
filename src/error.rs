#[derive(Debug)]
pub enum AsmError {
    InvalidInstruction { line: usize, instruction: String },
    SyntaxError { line: usize, message: String },
}

impl std::error::Error for AsmError {}

impl std::fmt::Display for AsmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AsmError::InvalidInstruction { line, instruction } => {
                write!(f, "Invalid instruction on line {line}: {instruction}")
            }
            AsmError::SyntaxError { line, message } => {
                write!(f, "Syntax error on line {line}: {message}")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_asm_error_display() {
        let error = AsmError::InvalidInstruction {
            line: 1,
            instruction: "MOV A, B".to_string(),
        };
        assert_eq!(error.to_string(), "Invalid instruction on line 1: MOV A, B");

        let error = AsmError::SyntaxError {
            line: 2,
            message: "Expected a comma".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Syntax error on line 2: Expected a comma"
        );
    }
}
