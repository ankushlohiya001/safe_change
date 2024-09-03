use clap::Parser;
use std::{
    fs::{copy, remove_file, File},
    io::{stdout, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

const BACKUP_SUFFIX: &str = ".bak";

/// CLI app to safely execute unsafe code (which might cause unaccessibility), by reverting changes if no response recieved.
#[derive(Parser)]
struct Args {
    /// Command to execute safely
    command: String,
    /// File which encounter changes (ie. to backup)
    file_path: String,
    /// timer after to revert changes
    #[arg(short, long, default_value_t = 30)]
    timer: u64,
    /// Command to execute after revert
    #[arg(default_value = "")]
    revert_command: String,
}

fn main() {
    let args = Args::parse();

    let backup_file = format!("{}{BACKUP_SUFFIX}", &args.file_path);
    // backing up file before executing command
    copy(&args.file_path, &backup_file).unwrap();

    // executing command itself
    let mut cmd = Command::new("sh");
    cmd.args(["-c", &args.command]);
    let output = cmd.output().unwrap();
    stdout().write(&output.stdout);

    //waiting for specified time (30 sec)
    sleep(Duration::from_secs(args.timer));

    //checking for file existence
    let file = File::open(&backup_file);
    if (file.is_ok()) {
        copy(&backup_file, args.file_path);
        remove_file(backup_file);

        // if post_revert specified execute it
        if args.revert_command.len() > 0 {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &args.revert_command]);
            let output = cmd.output().unwrap();
            stdout().write(&output.stdout);
        }
        println!("Changes reverted since backup file exists.");
    } else {
        println!("Nothing to change, since backup file deleted.");
    }
}
