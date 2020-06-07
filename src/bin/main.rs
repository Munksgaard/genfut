use genfut::{genfut, Opt};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    genfut(opt)?;

    Ok(())
}
