use std::path::Path;

pub fn cmd_sqlite3_dump<P: AsRef<Path>>(db_path: P, output_path: P) -> anyhow::Result<()> {
    let outfile = std::fs::File::create(output_path)?;

    let output = std::process::Command::new("sqlite3")
        .arg(db_path.as_ref())
        .arg(".dump")
        .stdout(std::process::Stdio::from(outfile))
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to execute sqlite3 command"))
    }

    Ok(())
}
