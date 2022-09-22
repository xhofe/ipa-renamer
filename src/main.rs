use clap::Parser;
fn main() -> anyhow::Result<()> {
    let renamer = ipa_renamer::Renamer::parse();
    renamer.run()?;
    Ok(())
}
