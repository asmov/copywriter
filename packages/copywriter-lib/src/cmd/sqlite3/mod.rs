use std::path::Path;
use std::process::{Command, Stdio};

pub fn dump_db<P: AsRef<Path>>(db_path: P, output_path: P) -> anyhow::Result<()> {
    let outfile = std::fs::File::create(output_path)?;

    let output = Command::new("sqlite3")
        .arg(db_path.as_ref())
        .arg(".dump")
        .stdout(Stdio::from(outfile))
        .output()
        .or_else(|e| anyhow::bail!("Unable to run `sqlite3` :: {}", e))?;

    if !output.status.success() {
        anyhow::bail!("Failed to execute sqlite3 database dump :: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
