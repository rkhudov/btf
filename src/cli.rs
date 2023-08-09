use structopt::StructOpt;
use std::path::PathBuf;

fn validate_cells(s: &str) -> Result<usize, String> {
    let value = s.parse::<usize>().map_err(|e| e.to_string())?;
    if value == 0 {
        return Err(String::from("The value must be non-zero."));
    }
    Ok(value)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "bft")]
pub struct Args {
    #[structopt(
        required(true),
        name = "PROGRAM",
        help = "The file of BF program to be parsed.",
        parse(from_os_str)
    )]
    pub program: PathBuf,

    #[structopt(
        short,
        long,
        help = "The size of VM's tape.",
        parse(try_from_str = validate_cells)
    )]
    pub cells: Option<usize>,

    #[structopt(
        short,
        long,
        help = "Whether to extend VM's tape or not. By default - false."
    )]
    pub extensible: Option<bool>,
}
