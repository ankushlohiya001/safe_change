use std::{
    env,
    fs::{copy, remove_file, File},
    io::{stdout, Write},
    process::Command,
    thread::sleep,
    time::Duration,
};

const BACKUP_SUFFIX: &str = ".bak";

fn main() {
    let command = env::args().nth(1).unwrap();
    let file_path = env::args().nth(2).unwrap();

    let backup_file = format!("{file_path}{BACKUP_SUFFIX}");
    // backing up file before executing command
    copy(&file_path, &backup_file).unwrap();

    // executing command itself
    let mut cmd = Command::new("sh");
    cmd.args(["-c", &command]);
    let output = cmd.output().unwrap();
    stdout().write(&output.stdout);

    //waiting for specified time (30 sec)
    sleep(Duration::from_secs(30));

    //checking for file existence
    let file = File::open(&backup_file);
    if (file.is_ok()) {
        copy(&backup_file, file_path);
        remove_file(backup_file);
        println!("Changes reverted since backup file exists.");
    } else {
        println!("Nothing to change, since backup file deleted.");
    }
}
