use std::{path::Path, process::Command};

pub fn run_rotor(
    selfie_dir: &Path,
    rotor_args: &str,
    make_target: &Option<String>,
) -> anyhow::Result<()> {
    // make sure we start fresh
    Command::new("make")
        .arg("clean")
        .current_dir(selfie_dir)
        .spawn()?
        .wait()?;

    let make_target = make_target.as_deref().unwrap_or("rotor-symbolic");

    Command::new("make")
        .arg(make_target)
        .arg(format!("rotor={}", rotor_args))
        .current_dir(selfie_dir)
        .spawn()?
        .wait()?;

    Ok(())
}
