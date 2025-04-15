use crate::*;

pub fn run() {
    match try_run() {
        Ok(_) => (),
        Err(e) => {
            match e.source() {
                Some(cause) => eprintln!("error: {} :: {}", e, cause),
                None => eprintln!("error: {}", e),
            }

            std::process::exit(1);
        }
    }
}

pub fn try_run() -> anyhow::Result<()> {
    let project = Project::locate()?;
    dbg!(project);
    Ok(())
}
