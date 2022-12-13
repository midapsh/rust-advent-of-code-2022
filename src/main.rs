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
        // Day 3
        "3-1" => println!("{}", p3_1()),
        "3-2" => println!("{}", p3_2()),
        // Day 4
        "4-1" => println!("{}", p4_1()),
        "4-2" => println!("{}", p4_2()),
        // Day 5
        "5-1" => println!("{}", p5_1()),
        "5-2" => println!("{}", p5_2()),
        // Day 6
        "6-1" => println!("{}", p6_1()),
        "6-2" => println!("{}", p6_2()),
        // Day 7
        "7-1" => println!("{}", p7_1()),
        "7-2" => println!("{}", p7_2()),
        // Day 8
        "8-1" => println!("{}", p8_1()),
        "8-2" => println!("{}", p8_2()),
        // Day 9
        "9-1" => println!("{}", p9_1()),
        "9-2" => println!("{}", p9_2()),
        // Day 10
        "10-1" => println!("{}", p10_1()),
        "10-2" => println!("{}", p10_2()),
        // Day 11
        "11-1" => println!("{}", p11_1()),
        "11-2" => println!("{}", p11_2()),
        _ => return Err(anyhow!("Unknown puzzle")),
    }
    Ok(())
}
