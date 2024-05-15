use std::{fs::OpenOptions, io::Read, path::Path, process::Command};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hyperfine {
    pub results: Vec<HyperfineResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperfineResult {
    pub command: String,
    pub mean: f64,
    pub stddev: f64,
    pub median: f64,
    pub user: f64,
    pub system: f64,
    pub min: f64,
    pub max: f64,
    pub times: Vec<f64>,
    pub exit_codes: Vec<i32>,
}

pub fn run(
    path: impl AsRef<Path>,
    hyperfine_output: impl AsRef<Path>,
    hyperfine_json_path: impl AsRef<Path>,
    timeout: Option<u128>,
) -> anyhow::Result<Hyperfine> {
    let json_path = hyperfine_json_path.as_ref();
    let mut json_out = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .read(true)
        .open(json_path)?;

    let mut btormc_cmd = format!("btormc {} -kmax 200", path.as_ref().display());

    if let Some(timeout) = timeout {
        btormc_cmd = format!("timeout --foreground {}s {}", timeout, btormc_cmd);
    }

    let _ = Command::new("hyperfine")
        .args(["--warmup", "3"])
        .args(["--runs", "5"])
        .arg("--ignore-failure")
        .arg("--export-json")
        .arg(json_path)
        .args([
            "--output",
            hyperfine_output
                .as_ref()
                .to_str()
                .context("Invalid output from 'btormc'")?,
        ])
        .arg(&btormc_cmd)
        .spawn()?
        .wait()?;

    let hyperfine: Hyperfine = serde_json::from_reader(&json_out).map_err(|_| {
        let mut output = String::new();
        json_out.read_to_string(&mut output).unwrap_or_default();
        anyhow::format_err!("Failed reading json_output: \n{}\n", output)
    })?;

    Ok(hyperfine)
}
