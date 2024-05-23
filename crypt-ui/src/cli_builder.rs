use crate::prelude::{self};
use clap::{arg, command, Arg, Command};
use crypt_cloud::crypt_core::{
    common::{get_machine_name, send_information},
    config::{self, ConfigTask, ItemsTask},
    db::import_keeper,
};
use std::path::PathBuf;

use crate::directive::{
    self, dropbox_download, dropbox_upload, dropbox_view, google_download, google_view,
};

pub fn start_cli() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .arg_required_else_help(true)
        .about("Upload or download files and folders from cloud providers while keeping your files, yours.")
        .subcommand(Command::new("config"))
        .subcommand(Command::new("decrypt"))
        .subcommand(Command::new("encrypt"))
        .subcommand(
            Command::new("dropbox")
            .arg_required_else_help(true)
                .hide(true)
                .about("upload, download, or view files and folders in your configured Dropbox")
                .arg(arg!(-u --upload  "Upload a file or folder").exclusive(true))
                .arg(arg!(-d --download  "Download a file or folder").exclusive(true))
                .arg(arg!(-v --view  "View files").alias("list").exclusive(true)),
        )
        .subcommand(
            Command::new("google")
            .arg_required_else_help(true)
            .about("upload, download, or view files and folders in your configured Drive")
                .arg(arg!(-u --upload  "Upload a file or folder").exclusive(true))
                .arg(arg!(-d --download  "Download a file or folder").exclusive(true))
                .arg(arg!(-v --view  "View files").alias("list").exclusive(true)),
        )

        .subcommand(Command::new("keeper"))

        .subcommand(Command::new("ls")
            .arg_required_else_help(true)
            .about("upload, download, or view files and folders in your configured Drive")
            .arg(arg!(-l --local   "Show all files contained in the local crypt folder").conflicts_with("cloud"))
            .arg(arg!(-c --cloud  "Show all files contained in the cloud folder"))
        )

        .subcommand(
            Command::new("test")
            .short_flag('t')
            .long_flag("test")
            .about("ssshhhh, personal testing only")
            .hide(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("test", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        _ => (),
    }
}
