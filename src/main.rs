//! Provide implementation of parsing BF program.
use argh::FromArgs;
use btf_interp::VirtualMachine;
use btf_types::BrainFuckProgram;
use std::error::Error;
use std::path::PathBuf;

#[derive(FromArgs)]
#[argh(description = "brainfuck program parser", name = "bft")]
struct Args {
    #[argh(option, description = "brainfuck program to be parsed.")]
    program: PathBuf,
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = argh::from_env();
    let file_path = args.program;
    // let file_path = args_os().nth(1).expect("Please, provide file path.");
    let bf_program = BrainFuckProgram::from_file(file_path);
    let vm: VirtualMachine<u8> = VirtualMachine::new(None, None);
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
