use std::fmt;

#[derive(Debug, Clone)]
pub enum AsmError {
    InvalidInstruction { line: usize, instruction: String },
    SyntaxError { line: usize, message: String },
}

impl std::error::Error for AsmError {}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl Clone for AssemblyError {
    fn clone(&self) -> Self {
        match self {
            AssemblyError::AsmErrors(errors) => AssemblyError::AsmErrors(errors.clone()),
            AssemblyError::Other(_) => AssemblyError::Other(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Cannot clone Box<dyn Error>"))),
        }
    }
}


#[derive(Debug)]
pub enum AssemblyError {
    AsmErrors(Vec<AsmError>),
    Other(Box<dyn std::error::Error>),
}

impl From<Box<dyn std::error::Error>> for AssemblyError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        AssemblyError::Other(error)
    }
}

impl From<std::io::Error> for AssemblyError {
    fn from(error: std::io::Error) -> Self {
        AssemblyError::Other(Box::new(error))
    }
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyError::AsmErrors(errors) => {
                write!(f, "Assembly encountered the following errors:\n")?;
                for error in errors {
                    writeln!(f, "  - {}", error)?;
                }
                Ok(())
            }
            AssemblyError::Other(err) => write!(f, "{}", err),
        }
    }
}


impl std::error::Error for AssemblyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AssemblyError::Other(err) => Some(err.as_ref()),
            _ => None,
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
