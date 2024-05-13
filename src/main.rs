use hack_assembler::{match_args, run};

fn main() {
    let config = match_args().unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Error running the Hack Assembler: {}", err);
        std::process::exit(1);
    }
}
