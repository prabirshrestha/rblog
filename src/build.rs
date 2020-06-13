use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let mut ructe = Ructe::from_env()?;
    ructe.statics()?.add_files("static")?;
    ructe.compile_templates("templates")?;
    Ok(())
}
