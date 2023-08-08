use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

/// Provide enum for all possible BF language instructions.
#[derive(Debug)]
pub enum RawInstructions {
    /// Reprsent `>` symbol. Increment the data pointer by one (to point to the next cell to the right).
    IncrementDataPointer,
    /// Reprsent `<` symbol. Decrement the data pointer by one (to point to the next cell to the left).
    DecrementDataPointer,
    /// Reprsent `+` symbol. Increment the byte at the data pointer by one.
    IncrementByte,
    // Reprsent `-` symbol. Decrement the byte at the data pointer by one.
    DecrementByte,
    /// Reprsent `.` symbol. Output the byte at the data pointer.
    OutputByte,
    /// Reprsent `,` symbol. Accept one byte of input, storing its value in the byte at the data pointer.
    AcceptByte,
    /// Reprsent `[` symbol. If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
    ZeroJump,
    /// Reprsent `]` symbol. If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.
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

/// Provide structure to reprsent location of BF instruction in file.
#[derive(Debug)]
pub struct IntructionPosition {
    /// BF instruction.
    instruction: RawInstructions,
    /// Line of the file from where BF instruction is parsed.
    line: usize,
    /// Positoin at the line from where BF instruction is parsed.
    position: usize,
}

impl fmt::Display for IntructionPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[inputfile:{}:{}] {}",
            self.line, self.position, self.instruction,
        )
    }
}

/// Provide structure to reprsent BF program.
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
            position += 1;
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
        }
        BrainFuckProgram {
            filename,
            instructions,
        }
    }

    /// Get name of the file from where BF program is parsed.
    pub fn filename(&self) -> &String {
        &self.filename
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
