use std::io::{BufRead, Seek, Write};

use crate::{instruction::Instruction, AsmError, Config, Result, SymbolTable};

pub struct Assembler<R: BufRead, W: Write> {
    config: Config,
    reader: R,
    writer: W,
    symbol_table: SymbolTable,
    errors: Vec<AsmError>,
    current_address: u16,
    line_number: usize,
}

impl<R: BufRead + Seek, W: Write> Assembler<R, W> {
    pub fn new(reader: R, writer: W, config: Config, symbol_table: SymbolTable) -> Self {
        Assembler {
            config,
            reader,
            writer,
            symbol_table,
            errors: Vec::new(),
            current_address: 0,
            line_number: 0,
        }
    }

    pub fn assemble(&mut self) -> Result<()> {
        self.build_symbol_table()?;
        self.reader.seek(std::io::SeekFrom::Start(0))?;
        self.line_number = 1;
        let mut reader_lines = self.reader.by_ref().lines();

        let mut instructions: Vec<Instruction> = Vec::new();
        while let Some(line_result) = reader_lines.next() {
            let line = match line_result {
                Ok(line) => line,
                Err(e) => {
                    self.errors.push(AsmError::SyntaxError {
                        line: self.line_number,
                        message: format!("Failed to read line: {}", e),
                    });
                    continue;
                }
            };
            let sanitized = match sanitize_line(&line, self.line_number) {
                Ok(opt) => opt,
                Err(_) => continue,
            };

            if let Some(line) = sanitized {
                match Instruction::parse(&line, self.line_number) {
                    Ok(instruction) => instructions.push(instruction),
                    Err(_) => {} // Errors are already handled in first
                }
            }
            self.line_number += 1;
        }

        if !self.errors.is_empty() {
            println!("Errors encountered during assembly:");
            for error in &self.errors {
                println!("{}", error);
            }
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Assembly failed due to errors",
            )));
        }

        for instruction in instructions {
            self.handle_instruction(instruction)?;
        }

        println!(
            "Successfully assembled the file: {}",
            self.config.output_file.display()
        );
        self.writer.flush()?;

        Ok(())
    }
    fn handle_instruction(&mut self, instruction: Instruction) -> Result<()> {
        match instruction {
            Instruction::A(a_instruction) => {
                let address = a_instruction.value();
                writeln!(self.writer, "{:016b}", address)?;
            }
            Instruction::C(c_instruction) => {
                let binary = c_instruction.to_binary();
                writeln!(self.writer, "{}", binary)?;
            }
            Instruction::Variable(variable_name) => {
                let address = self.symbol_table.add_variable(variable_name.clone());
                let binary = format!("{:016b}", address);
                writeln!(self.writer, "{}", binary)?;
            }
            Instruction::L(_) => {}
        }
        Ok(())
    }

    fn build_symbol_table(&mut self) -> Result<()> {
        let mut reader_lines = self.reader.by_ref().lines();

        while let Some(line_result) = reader_lines.next() {
            self.line_number += 1;
            let line = line_result?;

            let sanitized_result = sanitize_line(&line, self.line_number);
            let sanitized = match sanitized_result {
                Ok(s) => s,
                Err(e) => {
                    self.errors.push(e);
                    continue;
                }
            };

            if let Some(ref line) = sanitized {
                let instruction_result = Instruction::parse(line, self.line_number);

                // build
                match instruction_result {
                    Ok(instruction) => {
                        if let Instruction::L(label) = instruction {
                            self.symbol_table.add_label(label, self.current_address);
                        } else {
                            self.current_address += 1;
                        }
                    }
                    Err(e) => self.errors.push(e),
                }
            }
        }
        Ok(())
    }
}

fn sanitize_line(line: &str, line_number: usize) -> std::result::Result<Option<String>, AsmError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if !trimmed.is_ascii() {
        return Err(AsmError::SyntaxError {
            line: line_number,
            message: "Non-ASCII characters are not allowed".to_string(),
        });
    }

    if trimmed.starts_with("//") {
        return Ok(None);
    }
    if trimmed.contains("//") {
        let comment_index = trimmed.find("//").unwrap();
        return Ok(Some(trimmed[..comment_index].trim().to_string()));
    }

    Ok(Some(trimmed.to_string()))
}
