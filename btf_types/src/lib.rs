use std::fs;
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub enum RawInstructions {
    IncrementDataPointer, // <
    DecrementDataPointer, // >
    IncrementByte,  // +
    DecrementByte,  // -
    OutputByte,  // .
    AcceptByte,  // ,
    ZeroJump,  // [
    NonZeroJump,  // ]
}

impl TryFrom<char> for RawInstructions {
    type Error = &'static str;

    fn try_from(symbol: char) -> Result<RawInstructions, Self::Error> {
        match symbol {
            '<' => Ok(RawInstructions::IncrementDataPointer),
            '>' => Ok(RawInstructions::DecrementDataPointer),
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

#[derive(Debug)]
pub struct BrainFuckProgram {
    filename: String,
    instructions: Vec<RawInstructions>,
}

impl BrainFuckProgram {
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
        BrainFuckProgram {filename, instructions}
    }

    fn from_file<T: AsRef<Path>>(file_path: T) -> Result<(), Box<dyn Error>> {
        let file_path_ref = file_path.as_ref();
        let content = fs::read_to_string(file_path_ref)?;
        let bf_program = Self::new(file_path_ref.to_string_lossy().to_string(), content);
        println!("{:?}", bf_program);
        Ok(())
    }
}
