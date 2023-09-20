//! Provide interpreter implementation for BF program.
use btf_types::BrainFuckProgram;
use std::num::NonZeroUsize;

/// Provide structure for Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine<T> {
    /// The collection to store elements of the tape.
    tape: Vec<T>,
    /// The size of the tape.
    tape_size: usize,
    /// Whether to allow adjust size of the tape of not.
    adjust_tape: bool,
    /// The pointer to the current element of tape.
    pointer: usize,
}

impl<T> VirtualMachine<T> {
    /// Create VM based on the size, by default is 30 000. Also, it can be adjusted, by default it doesn't.
    pub fn new(size: Option<NonZeroUsize>, adjust_tape: Option<bool>) -> Self {
        VirtualMachine {
            tape: Vec::new(),
            tape_size: size.map(NonZeroUsize::get).unwrap_or(3000),
            adjust_tape: adjust_tape.unwrap_or(false),
            pointer: 0,
        }
    }

    /// Add element into tape. If tape size exceeded, element is not going to be added, error message is shown.
    pub fn push(&mut self, item: T) -> Result<(), String> {
        if self.tape.len() >= self.tape_size {
            if !self.adjust_tape {
                return Err(format!("Max tape size of {} reached.", self.tape_size));
            }
            self.tape_size += 1;
        }
        self.tape.push(item);
        Ok(())
    }

    /// Interpreter BF program into human-readable format.
    pub fn interpreter(&self, bf_program: &BrainFuckProgram) {
        for instruction_position in bf_program.instructions() {
            println!(
                "[{}:{}",
                bf_program.filename().display(),
                instruction_position
            );
        }
    }
}
