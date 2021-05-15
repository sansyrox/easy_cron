use os_pipe::pipe;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub fn delete_all_jobs() {
    let mut crontab = Command::new("sh");
    crontab.arg("-c");
    crontab.arg("echo '' | crontab -");

    // Here's the interesting part. Open a pipe, copy its write end, and
    // give both copies to the child.
    let (mut reader, writer) = pipe().unwrap();
    let writer_clone = writer.try_clone().unwrap();
    crontab.stdout(writer);
    crontab.stderr(writer_clone);

    // Now start the child running.
    let mut handle = crontab.spawn().unwrap();

    // Very important when using pipes: This parent process is still
    // holding its copies of the write ends, and we have to close them
    // before we read, otherwise the read end will never report EOF. The
    // Command object owns the writers now, and dropping it closes them.
    drop(crontab);
    let mut output = String::new();
    reader.read_to_string(&mut output).unwrap();
    handle.wait().unwrap();
    println!("{}", output);
}

pub fn process_cron(second: &str, hour: &str, day: &str, month: &str, year: &str, command: &str) {
    let crontab = Command::new("crontab")
        .arg("-l")
        .output()
        .expect("failed to execute child");

    println!("{}", String::from_utf8_lossy(&crontab.stderr));
    let previous_jobs = String::from_utf8_lossy(&crontab.stdout);
    println!("{}", previous_jobs);

    let crontab_command = format!("{} {} {} {} {} {}", second, hour, day, month, year, command);
    let child = Command::new("echo")
        .arg(crontab_command)
        .output()
        .expect("failed to execute child");

    let output = child.stdout;

    println!("{}", String::from_utf8_lossy(&output))
}

pub fn current_jobs() -> String {
    let crontab = Command::new("crontab")
        .arg("-l")
        .output()
        .expect("failed to execute child");

    if !crontab.stderr.is_empty() {
        return String::from("No Jobs Running");
    }

    println!("{}", String::from_utf8_lossy(&crontab.stderr));
    let running_jobs = String::from_utf8_lossy(&crontab.stdout);

    return running_jobs.to_string();
}
