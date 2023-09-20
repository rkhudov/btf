use std::num::NonZeroUsize;
use std::path::PathBuf;
use structopt::StructOpt;

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

    #[structopt(short, long, help = "The size of VM's tape.")]
    pub cells: Option<NonZeroUsize>,

    #[structopt(
        short,
        long,
        help = "Whether to extend VM's tape or not. By default - false."
    )]
    pub extensible: Option<bool>,
}
