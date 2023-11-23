mod cloud_storage;
mod ui;
mod util;
use anyhow::{self, Ok, Result};
use ui::*;
use util::*;
use ui::tui::format_directory;
use util::path::{
    generate_directory, Directory,
};
use std::path::PathBuf;


fn main() -> Result<()> {
    //Load config file
    config::load_config();

    //Load user settings

    //let directory_tree = generate_directory("").unwrap();
    //let formatted_tree = format_directory(&directory_tree, &PathBuf::from("/"), 0);

    //println!("{}", formatted_tree);
    //Load the UI - CLI only currently
    //cli::load_cli();
    let _ = tui::load_tui();
    //gui::load_gui();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_path() {
        let dir = "../test_folder";
        for p in path::walk_directory(dir).unwrap() {
            let s = util::common::read_to_vec_string(p.as_str());
            println!("{:?} from file: {}", s, p);
        }
    }
}

fn _test_write_db() -> Result<()> {
    let t = util::parse::toml_example()?;
    println!("{:?}", t);
    parse::prepend_file(t, "db")
}
