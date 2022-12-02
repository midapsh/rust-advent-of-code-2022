use anyhow::anyhow;
use structopt::StructOpt;

use rust_advent_of_code_2022::*;

#[derive(StructOpt)]
struct Opts {
    part: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    match opts.part.as_str() {
        // Day 1
        "1-1" => println!("{}", p1_1()),
        "1-2" => println!("{}", p1_2()),
        // Day 2
        "2-1" => println!("{}", p2_1()),
        "2-2" => println!("{}", p2_2()),
        _ => return Err(anyhow!("Unknown puzzle")),
    }
    Ok(())
}
