//! Provide interpreter implementation for BF program.
use btf_types::{BrainFuckProgram, RawInstructions};
use std::io::{Read, Write};
use std::num::NonZeroUsize;

/// Provide trait for cell in Virtual Machine.
pub trait CellKind {
    /// Wrapper to increase value by 1 in the cell.
    fn wrapping_increment(&mut self);
    /// Wrapper to decrease value by 1 in the cell.
    fn wrapping_decrement(&mut self);
    /// Wrapper to set value in the cell.
    fn wrapping_set_value(&mut self, value: u8);
    /// Wrapper to get value from the cell.
    fn wrapping_get_value(&self) -> u8;
}

/// Provide implementation for u8 type cell in Virtual Machine.
impl CellKind for u8 {
    /// Implementation for u8 cell type of wrapper to increase value by 1 in it.
    fn wrapping_increment(&mut self) {
        *self += 1;
    }
    /// Implementation for u8 cell type of wrapper to decrease value by 1 in it.
    fn wrapping_decrement(&mut self) {
        *self -= 1;
    }
    /// Implementation for u8 cell type of wrapper to set value in it.
    fn wrapping_set_value(&mut self, value: u8) {
        *self = value;
    }
    /// Implementation for u8 cell type of wrapper to get value from it.
    fn wrapping_get_value(&self) -> u8 {
        *self
    }
}

/// Provide enum of errors for Virtual Machine.
#[derive(Debug, PartialEq)]
pub enum VMError {
    /// Represent the case when the lenght of the tape is exceeded.
    NextElementNotReachable { line: usize, position: usize },
    /// Represent the case when element before the first one is trying to be reached.
    PreviousElementNotReachanble { line: usize, position: usize },
    ///IO Error at current instruction
    IOError { line: usize, position: usize },
}

/// Provide structure for Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine<'a, T> {
    /// The collection to store elements of the tape.
    tape: Vec<T>,
    /// The size of the tape.
    tape_size: usize,
    /// Whether to allow adjust size of the tape of not.
    adjust_tape: bool,
    /// The pointer to the current element of tape.
    head: usize,
    /// Current instruction pointer
    instruction_pointer: usize,
    /// BrainFuck Program.
    program: &'a BrainFuckProgram,
}

impl<'a, T: CellKind> VirtualMachine<'a, T>
where
    T: CellKind,
{
    /// Create VM based on the size, by default is 30 000. Also, it can be adjusted, by default it doesn't.
    pub fn new(
        program: &'a BrainFuckProgram,
        size: Option<NonZeroUsize>,
        adjust_tape: Option<bool>,
    ) -> Self {
        VirtualMachine {
            tape: Vec::new(),
            tape_size: size.map(NonZeroUsize::get).unwrap_or(3000),
            adjust_tape: adjust_tape.unwrap_or(false),
            head: 0,
            instruction_pointer: 0,
            program,
        }
    }

    /// Interpreter BF program into human-readable format.
    pub fn interpreter(&self) {
        for instruction_position in self.program.instructions() {
            println!(
                "[{}:{}",
                self.program.filename().display(),
                instruction_position
            );
        }
    }

    /// Go to the next element in tape. If tape size exceeded, error message is shown.
    fn next_element(&mut self) -> Result<usize, VMError> {
        self.instruction_pointer += 1;
        if self.head + 1 == self.tape_size {
            let instruction = &self.program.instructions()[self.head];
            return Err(VMError::NextElementNotReachable {
                line: instruction.line(),
                position: instruction.position(),
            });
        }
        self.head += 1;
        Ok(self.instruction_pointer)
    }

    /// Go to the previous element in tape. If it is the first element, error message is shown.
    fn previous_element(&mut self) -> Result<usize, VMError> {
        self.instruction_pointer += 1;
        if self.head == 0 {
            let instruction = &self.program.instructions()[self.head];
            return Err(VMError::PreviousElementNotReachanble {
                line: instruction.line(),
                position: instruction.position(),
            });
        }
        self.head -= 1;
        Ok(self.instruction_pointer)
    }

    /// Add 1 to the element where head is pointing to.
    fn wrapped_add(&mut self) -> Result<usize, VMError> {
        self.instruction_pointer += 1;
        self.tape[self.head].wrapping_increment();
        Ok(self.instruction_pointer)
    }

    /// Substract 1 to the element where head is pointing to.
    fn wrapped_sub(&mut self) -> Result<usize, VMError> {
        self.instruction_pointer += 1;
        self.tape[self.head].wrapping_decrement();
        Ok(self.instruction_pointer)
    }

    /// Basic IO read.
    fn read(&mut self, reader: &mut impl Read) -> Result<usize, VMError> {
        self.instruction_pointer += 1;
        let mut buffer = [0; 1];
        match reader.read_exact(&mut buffer) {
            Ok(_) => self.tape[self.head].wrapping_set_value(buffer[0]),
            Err(_err) => {
                let instruction = &self.program.instructions()[self.head];
                return Err(VMError::IOError {
                    line: instruction.line(),
                    position: instruction.position(),
                });
            }
        }
        Ok(self.instruction_pointer)
    }

    /// Basic IO write.
    fn output(&mut self, writer: &mut impl Write) -> Result<usize, VMError> {
        self.instruction_pointer += 1;
        match writer.write_all(&[self.tape[self.head].wrapping_get_value()]) {
            Ok(_) => match writer.flush() {
                Ok(_) => Ok(self.instruction_pointer),
                Err(_err) => {
                    let instruction = &self.program.instructions()[self.head];
                    Err(VMError::IOError {
                        line: instruction.line(),
                        position: instruction.position(),
                    })
                }
            },
            Err(_err) => {
                let instruction = &self.program.instructions()[self.head];
                Err(VMError::IOError {
                    line: instruction.line(),
                    position: instruction.position(),
                })
            }
        }
    }

    /// Not make jump
    fn non_zero_jump(&self, index: usize) -> Result<usize, VMError> {
        Ok(index)
    }

    /// Make jump
    fn zero_jump(&mut self, index: usize) -> Result<usize, VMError> {
        let current_value = &self.tape[self.instruction_pointer];

        if current_value.wrapping_get_value() == 0 {
            Ok(self.instruction_pointer + 1)
        } else {
            Ok(index + 1)
        }
    }

    /// Make interpretation of the BF program
    pub fn interpret<R, W>(&mut self, input: &mut R, output: &mut W) -> Result<(), VMError>
    where
        R: Read,
        W: Write,
    {
        loop {
            let current_pointer = self.instruction_pointer;
            let current_instruction = self.program.instructions()[current_pointer].instruction();
            let next_pointer = match current_instruction {
                RawInstructions::IncrementByte => self.wrapped_add(),
                RawInstructions::DecrementByte => self.wrapped_sub(),
                RawInstructions::IncrementDataPointer => self.next_element(),
                RawInstructions::DecrementDataPointer => self.previous_element(),
                RawInstructions::OutputByte => self.output(output),
                RawInstructions::AcceptByte => self.read(input),
                RawInstructions::NonZeroJump => self.non_zero_jump(current_pointer),
                RawInstructions::ZeroJump => self.zero_jump(current_pointer),
            }?;

            if next_pointer == self.program.instructions().len() {
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BrainFuckProgram;
    use crate::NonZeroUsize;
    use crate::VMError;
    use crate::VirtualMachine;

    use std::fs::File;
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn test_default_new_vm() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let tmp_file = File::create(&file_path).unwrap();

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let default_vm: VirtualMachine<u8> = VirtualMachine::new(&program, None, None);
        assert_eq!(default_vm.tape_size, 3000);
        assert_eq!(default_vm.tape.len(), 0);
        assert_eq!(default_vm.head, 0);
        assert!(!default_vm.adjust_tape);

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }

    #[test]
    fn test_new_vm() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let tmp_file = File::create(&file_path).unwrap();

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let vm: VirtualMachine<u8> =
            VirtualMachine::new(&program, NonZeroUsize::new(100), Some(true));
        assert_eq!(vm.tape_size, 100);
        assert_eq!(vm.tape.len(), 0);
        assert_eq!(vm.head, 0);
        assert!(vm.adjust_tape);

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }

    #[test]
    fn test_failed_to_get_previous_element_vm() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let mut tmp_file = File::create(&file_path).unwrap();
        let _ = writeln!(tmp_file, "+[-");

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, NonZeroUsize::new(1), None);
        assert_eq!(
            vm.previous_element(),
            Err(VMError::PreviousElementNotReachanble {
                line: 1,
                position: 1
            })
        );

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }

    #[test]
    fn test_get_previous_element_vm() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let tmp_file = File::create(&file_path).unwrap();

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, None, None);
        assert_eq!(vm.next_element(), Ok(1));
        assert_eq!(vm.previous_element(), Ok(2));

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }

    #[test]
    fn test_failed_to_get_next_element_vm() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let mut tmp_file = File::create(&file_path).unwrap();
        let _ = writeln!(tmp_file, "+[-");

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, NonZeroUsize::new(3), None);
        let _ = vm.next_element();
        let _ = vm.next_element();
        assert_eq!(
            vm.next_element(),
            Err(VMError::NextElementNotReachable {
                line: 1,
                position: 3,
            })
        );

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }

    #[test]
    fn test_next_element_vm() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let tmp_file = File::create(&file_path).unwrap();

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, None, None);
        assert_eq!(vm.next_element(), Ok(1));

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }
}
