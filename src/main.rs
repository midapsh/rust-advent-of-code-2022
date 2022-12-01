use rust_advent_of_code_2022::*;
use anyhow::anyhow;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    match opts.part.as_str() {
        "1-1" => println!("{}", p1_1()),
        _ => return Err(anyhow!("Unknown puzzle")),
    }
    Ok(())
}