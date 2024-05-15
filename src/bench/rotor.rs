use std::{path::Path, process::Command};

pub fn run_rotor(selfie_dir: &Path, rotor_args: &str) -> anyhow::Result<()> {
    // make sure we start fresh
    Command::new("make")
        .arg("clean")
        .current_dir(selfie_dir)
        .spawn()?
        .wait()?;

    Command::new("make")
        .arg("rotor-symbolic")
        .arg(format!("rotor={}", rotor_args))
        .current_dir(selfie_dir)
        .spawn()?
        .wait()?;

    Ok(())
}
