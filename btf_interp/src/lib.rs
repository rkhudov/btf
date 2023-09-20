//! Provide interpreter implementation for BF program.
use btf_types::BrainFuckProgram;
use std::num::NonZeroUsize;

pub trait CellKind {
    fn wrapped_add(&mut self, v: &Self) -> Self;
    fn wrapped_sub(&mut self, v: &Self) -> Self;
}

impl CellKind for u8 {
    fn wrapped_add(&mut self, v: &Self) -> Self {
        *self += v;
        *self
    }
    fn wrapped_sub(&mut self, v: &Self) -> Self {
        *self -= v;
        *self
    }
}

/// Provide enum of errors for Virtual Machine.
#[derive(Debug, PartialEq)]
pub enum VMError {
    /// Represent the case when the lenght of the tape is exceeded.
    NextElementNotReachable,
    /// Represent the case when element before the first one is trying to be reached.
    PreviousElementNotReachanble,
}

/// Provide structure for Virtual Machine
#[derive(Debug)]
pub struct VirtualMachine<'a, CellKind> {
    /// The collection to store elements of the tape.
    tape: Vec<CellKind>,
    /// The size of the tape.
    tape_size: usize,
    /// Whether to allow adjust size of the tape of not.
    adjust_tape: bool,
    /// The pointer to the current element of tape.
    head: usize,
    program: &'a BrainFuckProgram,
}

impl<'a, T> VirtualMachine<'a, T> {
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
    pub fn next_element(&mut self) -> Result<(), VMError> {
        if self.head == self.tape_size - 1 {
            return Err(VMError::NextElementNotReachable);
        }
        self.head += 1;
        Ok(())
    }

    /// Go to the previous element in tape. If it is the first element, error message is shown.
    pub fn previous_element(&mut self) -> Result<(), VMError> {
        if self.head == 0 {
            return Err(VMError::PreviousElementNotReachanble);
        }
        self.head -= 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::BrainFuckProgram;
    use crate::NonZeroUsize;
    use crate::VMError;
    use crate::VirtualMachine;

    use std::fs::File;
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
        let tmp_file = File::create(&file_path).unwrap();

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, NonZeroUsize::new(1), None);
        assert_eq!(
            vm.previous_element(),
            Err(VMError::PreviousElementNotReachanble)
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
        assert_eq!(vm.next_element(), Ok(()));
        assert_eq!(vm.previous_element(), Ok(()));

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }

    #[test]
    fn test_failed_to_get_next_element_vm() {
        let tmp_dir = TempDir::new("example").unwrap();
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let tmp_file = File::create(&file_path).unwrap();

        let program = BrainFuckProgram::from_file(&file_path).unwrap();

        let mut vm: VirtualMachine<u8> = VirtualMachine::new(&program, NonZeroUsize::new(1), None);
        assert_eq!(vm.next_element(), Err(VMError::NextElementNotReachable));

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
        assert_eq!(vm.next_element(), Ok(()));

        drop(tmp_file);
        tmp_dir.close().unwrap();
    }
}
