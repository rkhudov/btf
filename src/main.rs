//! Provide implementation of parsing BF program.
mod cli;
use btf_interp::VirtualMachine;
use btf_types::BrainFuckProgram;
use cli::Args;
use std::error::Error;
use std::process::{exit, ExitCode};
use structopt::StructOpt;

fn run_bft(args: Args) -> Result<(), Box<dyn Error>> {
    let bf_program = BrainFuckProgram::from_file(args.program);
    match bf_program {
        Ok(bf_program) => {
            bf_program.validate_brackets()?;
            let vm: VirtualMachine<u8> = VirtualMachine::new(args.cells, args.extensible);
            vm.interpreter(&bf_program);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let args = cli::Args::from_args();
    match run_bft(args) {
        Ok(_smth) => exit(0),
        Err(e) => {
            eprintln!("btf: {}", e);
            exit(1);
        }
    }
}
