//! Provide types implementation for BF interpreter.
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

/// Provide enum for all possible BF language instructions.
#[derive(Debug)]
pub enum RawInstructions {
    /// Represent `>` symbol. Increment the data pointer by one (to point to the next cell to the right).
    IncrementDataPointer,
    /// Represent `<` symbol. Decrement the data pointer by one (to point to the next cell to the left).
    DecrementDataPointer,
    /// Represent `+` symbol. Increment the byte at the data pointer by one.
    IncrementByte,
    // Represent `-` symbol. Decrement the byte at the data pointer by one.
    DecrementByte,
    /// Represent `.` symbol. Output the byte at the data pointer.
    OutputByte,
    /// Represent `,` symbol. Accept one byte of input, storing its value in the byte at the data pointer.
    AcceptByte,
    /// Represent `[` symbol. If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
    ZeroJump,
    /// Represent `]` symbol. If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
    NonZeroJump,
}

/// Try to convert char into BF language instruction.
impl TryFrom<char> for RawInstructions {
    type Error = &'static str;

    fn try_from(symbol: char) -> Result<RawInstructions, Self::Error> {
        match symbol {
            '<' => Ok(RawInstructions::DecrementDataPointer),
            '>' => Ok(RawInstructions::IncrementDataPointer),
            '+' => Ok(RawInstructions::IncrementByte),
            '-' => Ok(RawInstructions::DecrementByte),
            '.' => Ok(RawInstructions::OutputByte),
            ',' => Ok(RawInstructions::AcceptByte),
            '[' => Ok(RawInstructions::ZeroJump),
            ']' => Ok(RawInstructions::NonZeroJump),
            _ => Err("Non BF instruction"),
        }
    }
}

/// Provide human-readable format of the instructions.
impl fmt::Display for RawInstructions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RawInstructions::IncrementDataPointer => write!(f, "Increment data pointer"),
            RawInstructions::DecrementDataPointer => write!(f, "Decrement data pointer"),
            RawInstructions::IncrementByte => write!(f, "Increment byte"),
            RawInstructions::DecrementByte => write!(f, "Decrement byte"),
            RawInstructions::OutputByte => write!(f, "Output byte"),
            RawInstructions::AcceptByte => write!(f, "Accept byte"),
            RawInstructions::ZeroJump => write!(f, "Zero jump"),
            RawInstructions::NonZeroJump => write!(f, "Non zero jump"),
        }
    }
}

/// Provide structure to represent location of BF instruction in file.
#[derive(Debug)]
pub struct IntructionPosition {
    /// BF instruction.
    instruction: RawInstructions,
    /// Line of the file from where BF instruction is parsed.
    line: usize,
    /// Positoin at the line from where BF instruction is parsed.
    position: usize,
}

impl IntructionPosition {
    /// Get parsed instruction.
    pub fn instruction(&self) -> &RawInstructions {
        &self.instruction
    }

    /// Get line of parsed instruction.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get position at the line of parsed instruction.
    pub fn position(&self) -> usize {
        self.position
    }
}

/// Provide human-readable format of the instruction with position in parsed file.
impl fmt::Display for IntructionPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}] {}", self.line, self.position, self.instruction)
    }
}

/// Provide structure to represent BF program.
#[derive(Debug)]
pub struct BrainFuckProgram {
    /// Name of the file from where program is parsed.
    filename: String,
    /// List of instructions with location parsed from file.
    instructions: Vec<IntructionPosition>,
}

impl BrainFuckProgram {
    /// Create BF program based on the name of the file and it's content.
    fn new(filename: String, content: String) -> Self {
        let mut instructions: Vec<IntructionPosition> = Vec::new();

        let mut line: usize = 1;
        let mut position: usize = 1;
        for char in content.chars() {
            if char == '\n' {
                line += 1;
                position = 0;
            }
            match RawInstructions::try_from(char) {
                Ok(instruction) => {
                    let instruction_position = IntructionPosition {
                        instruction,
                        line,
                        position,
                    };
                    instructions.push(instruction_position);
                }
                Err(_e) => {}
            }
            position += 1;
        }
        BrainFuckProgram {
            filename,
            instructions,
        }
    }

    /// Get name of the file from where BF program is parsed.
    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    /// Get list of instructions for BF program.
    pub fn instructions(&self) -> &[IntructionPosition] {
        &self.instructions[..]
    }

    /// Parse BF program from file.
    pub fn from_file<T: AsRef<Path>>(file_path: T) -> Result<BrainFuckProgram, Box<dyn Error>> {
        let file_path_ref = file_path.as_ref();
        let content = fs::read_to_string(file_path_ref)?;
        let bf_program = Self::new(file_path_ref.to_string_lossy().to_string(), content);
        Ok(bf_program)
    }
}

#[cfg(test)]
mod tests {
    use crate::BrainFuckProgram;

    #[test]
    fn test_new_bf() {
        let test_filename = "test_filename".to_string();
        let test_content = "sometext\n><+-.,[]\ncomment <".to_string();
        let bf_program = BrainFuckProgram::new(test_filename.clone(), test_content);
        assert_eq!(
            bf_program.filename(),
            test_filename.clone(),
            "Filename has to be {}.",
            test_filename.clone(),
        );
        assert_eq!(
            bf_program.instructions().len(),
            9,
            "Number of parsed instructions have to be 9",
        );
        assert_eq!(bf_program.instructions().get(0).unwrap().line(), 2); // the ugliest test that I've ever written
        assert_eq!(bf_program.instructions().get(0).unwrap().position(), 1);
        assert_eq!(bf_program.instructions().get(1).unwrap().line(), 2);
        assert_eq!(bf_program.instructions().get(1).unwrap().position(), 2);
        assert_eq!(bf_program.instructions().get(2).unwrap().line(), 2);
        assert_eq!(bf_program.instructions().get(2).unwrap().position(), 3);
        assert_eq!(bf_program.instructions().get(3).unwrap().line(), 2);
        assert_eq!(bf_program.instructions().get(3).unwrap().position(), 4);
        assert_eq!(bf_program.instructions().get(4).unwrap().line(), 2);
        assert_eq!(bf_program.instructions().get(4).unwrap().position(), 5);
        assert_eq!(bf_program.instructions().get(5).unwrap().line(), 2);
        assert_eq!(bf_program.instructions().get(5).unwrap().position(), 6);
        assert_eq!(bf_program.instructions().get(6).unwrap().line(), 2);
        assert_eq!(bf_program.instructions().get(6).unwrap().position(), 7);
        assert_eq!(bf_program.instructions().get(7).unwrap().line(), 2);
        assert_eq!(bf_program.instructions().get(7).unwrap().position(), 8);
        assert_eq!(bf_program.instructions().get(8).unwrap().line(), 3);
        assert_eq!(bf_program.instructions().get(8).unwrap().position(), 9);
    }
}
