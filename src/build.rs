fn main() -> anyhow::Result<()> {
    let gitcl = vergen_gitcl::GitclBuilder::all_git()?;

    if let Err(error) = vergen::Emitter::default()
        .add_instructions(&gitcl)?
        .fail_on_error()
        .emit()
    {
        println!(
            "cargo:warning=Could not generate git version information: {:?}",
            error
        );
        println!("cargo:rustc-env=VERGEN_GIT_SHA=nogit");
    }

    let mut ructe = ructe::Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("assets")?;
    statics.add_sass_file("assets/stylesheets/style.scss")?;
    ructe.compile_templates("templates")?;

    Ok(())
}
