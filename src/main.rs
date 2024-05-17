use crypt_ui::cli::load_cli;
fn main() -> anyhow::Result<()> {
    let _ = load_cli();

    Ok(())
}
