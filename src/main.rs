use std::io::Read;

use clap::Parser;
use periscope::{
    btor::{self},
    Config,
};

fn main() -> Result<(), String> {
    let config = Config::parse();

    match config {
        Config::ParseWitness { file } => {
            let input: Box<dyn Read> = match file {
                Some(path) => Box::new(std::fs::File::open(path).unwrap()),
                None => Box::new(std::io::stdin()),
            };

            btor::interpret_btor_witness(input)?;
        }
    };

    Ok(())
}
