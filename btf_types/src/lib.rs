use std::error::Error;
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

/// Provide structure to reprsent BF program.
#[derive(Debug)]
pub struct BrainFuckProgram {
    /// Name of the file from where program is parsed.
    filename: String,
    /// List of instructions parsed from file.
    instructions: Vec<RawInstructions>,
}

impl BrainFuckProgram {
    /// Create BF program based on the name of the file and it's content.
    fn new(filename: String, content: String) -> BrainFuckProgram {
        let mut instructions: Vec<RawInstructions> = Vec::new();
        for char in content.chars() {
            match RawInstructions::try_from(char) {
                Ok(instruction) => {
                    instructions.push(instruction);
                }
                Err(_e) => {}
            }
        }
        BrainFuckProgram {
            filename,
            instructions,
        }
    }

    /// Parse BF program from file.
    pub fn from_file<T: AsRef<Path>>(file_path: T) -> Result<BrainFuckProgram, Box<dyn Error>> {
        let file_path_ref = file_path.as_ref();
        let content = fs::read_to_string(file_path_ref)?;
        let bf_program = Self::new(file_path_ref.to_string_lossy().to_string(), content);
        Ok(bf_program)
    }

    /// Get name of the file from where BF program is parsed.
    pub fn filename(&self) -> &String {
        &self.filename
    }

    /// Get list of instructions for BF program.
    pub fn instructions(&self) -> &[RawInstructions] {
        &self.instructions[..]
    }
}
