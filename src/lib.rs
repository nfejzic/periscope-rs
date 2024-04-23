use std::path::PathBuf;

use clap::Parser;

pub mod btor;

#[derive(Debug, Clone, Parser)]
pub enum Config {
    /// Parse witness format of btormc generated from btor2 model. Parses from stdin if path to
    /// file is not provided.
    ParseWitness {
        /// Path to the witness file.
        file: Option<PathBuf>,
    },
}
