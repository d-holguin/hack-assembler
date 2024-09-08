mod assembler;
mod error;
mod instruction;
mod symbol_table;

use clap::{arg, Command};

use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

pub use error::AsmError;
pub use symbol_table::SymbolTable;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;



pub struct Config {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

pub fn match_args() -> Result<Config> {
    let matches = Command::new("Hack Assembler")
        .version("1.0")
        .author("d-holguin")
        .about("This is a assembler for the Hack computer, part of the Nand2Tetris course")
        .arg(arg!(-f --file [FILE] "Sets the input file").value_hint(clap::ValueHint::FilePath))
        .hide_possible_values(false)
        .get_matches();

    let input_file = matches
        .get_one::<String>("file")
        .ok_or("No input file provided")?;

    let input_file = Path::new(&input_file);

    if input_file.extension().unwrap_or_default() != "asm" {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "The provided file must have an .asm extension",
        )));
    }
    let output_file = input_file.with_extension("hack");

    Ok(Config {
        input_file: PathBuf::from(input_file),
        output_file: PathBuf::from(output_file),
    })
}

pub fn run(config: Config) -> Result<()> {
    let reader = BufReader::new(File::open(&config.input_file)?);
    let writer = BufWriter::new(File::create(&config.output_file)?);
    let symbol_table = SymbolTable::new();

    let mut assembler = assembler::Assembler::new(reader, writer, config, symbol_table);
    assembler.assemble()?;
    Ok(())
}
