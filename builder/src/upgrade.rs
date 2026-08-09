use std::process::Command;

use anyhow::bail;

use crate::util::run;

pub fn upgrade() -> anyhow::Result<()> {
    run("Create fresh commit", "jj new @-")?;
    run_and_commit("Upgrade direct dependencies", "cargo upgrade")?;
    run_and_commit("Upgrade transitive dependencies", "cargo update")?;

    Ok(())
}

fn run_and_commit(description: &str, command: &str) -> anyhow::Result<()> {
    run(description, command)?;

    if commit_has_changes()? {
        run(
            "Commit changes",
            &format!("jj commit --message \"{description}\""),
        )?;
    }

    Ok(())
}

fn commit_has_changes() -> anyhow::Result<bool> {
    let output = Command::new("jj")
        .args(["--revision", "@"])
        .arg("--no-graph")
        .args(["--template", "!empty"])
        .output()?;

    if !output.status.success() {
        bail!("`jj` command failed: {status:?}", status = output.status);
    }

    let stdout = String::from_utf8(output.stdout)?;
    let commit_has_changes = stdout.parse()?;

    Ok(commit_has_changes)
}
