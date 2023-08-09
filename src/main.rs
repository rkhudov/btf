//! Provide implementation of parsing BF program.
mod cli;
use btf_interp::VirtualMachine;
use btf_types::BrainFuckProgram;
use structopt::StructOpt;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Args::from_args();
    let file_path = args.program;
    let bf_program = BrainFuckProgram::from_file(file_path);
    let vm: VirtualMachine<u8> = VirtualMachine::new(args.cells, args.extensible);
    match bf_program {
        Ok(bf_program) => {
            vm.interpreter(&bf_program);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
    Ok(())
}
