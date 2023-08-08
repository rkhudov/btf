use btf_interp::VirtualMachine;
use btf_types::BrainFuckProgram;
use std::env::args_os;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = args_os().nth(1).expect("Please, provide file path.");
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
