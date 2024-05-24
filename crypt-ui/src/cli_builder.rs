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
        .arg_required_else_help(true)
        .propagate_version(true)
        .about("Upload or download files and folders from cloud providers while keeping your files, yours.")
        .subcommand(Command::new("config"))
        .subcommand(Command::new("decrypt"))
        .subcommand(Command::new("encrypt"))
        .subcommand(
            Command::new("dropbox")
                .hide(true)
                .arg_required_else_help(true)
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

        .subcommand(Command::new("keeper")
            .arg_required_else_help(true)
            .about("manage local database")
            .arg(arg!(-i --import <PATH>  "View or update the database path"))
            .arg(arg!(-e --export   "Export database to .csv file"))
            .subcommand(Command::new("purge")
                .arg_required_else_help(true)
                .about("Purge commands to handle database and cloud provider tokens")
                .arg(arg!(-t --token   "Purge ALL cloud tokens"))
                .arg(arg!(-d --database   "Purge database, CANNOT be undone!"))
            )
        )

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
        Some(("config", sub_matches)) => (),
        Some(("decrypt", sub_matches)) => (),
        Some(("encrypt", sub_matches)) => (),
        Some(("dropbox", sub_matches)) => (),
        Some(("google", sub_matches)) => (),
        Some(("keeper", sub_matches)) => (),
        Some(("ls", sub_matches)) => {
            let local = sub_matches.get_one::<bool>("local").unwrap_or(&false);
            let cloud = sub_matches.get_one::<bool>("cloud").unwrap_or(&false);
            directive::ls(local, cloud);
        }
        Some(("test", sub_matches)) => directive::test(),
        _ => (),
    }
}
