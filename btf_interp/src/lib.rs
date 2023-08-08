use btf_types::BrainFuckProgram;

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
    pub fn new(size: Option<usize>, adjust_tape: Option<bool>) -> Self {
        VirtualMachine {
            tape: Vec::new(),
            tape_size: size.unwrap_or(30000),
            adjust_tape: adjust_tape.unwrap_or(false),
            pointer: 0,
        }
    }

    /// Get the tape.
    pub fn tape(&self) -> &[T] {
        &self.tape[..]
    }

    /// Get the size of the tape.
    pub fn tape_size(&self) -> usize {
        self.tape_size
    }

    /// Get the indicator whether it is possible to adjust tape or not.
    pub fn adjust_tape(&self) -> bool {
        self.adjust_tape
    }

    /// Get pointer of the tape.
    pub fn pointer(&self) -> usize {
        self.pointer
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

    pub fn interpreter(&self, bf_program: &BrainFuckProgram) {
        for instruction_position in bf_program.instructions() {
            println!("{}", instruction_position);
        }
    }
}
