extern crate daemonize;

use clap::{App, Arg};
use std::process::Command;

fn process_cron(second: &str, hour: &str, day: &str, month: &str, year: &str) {
    let crontab = Command::new("crontab")
        .arg("-l")
        .output()
        .expect("failed to execute child");

    println!("{}", String::from_utf8_lossy(&crontab.stderr));
    println!("{}", String::from_utf8_lossy(&crontab.stdout));

    let crontab_command = format!("{} {} {} {} {}", second, hour, day, month, year);
    let child = Command::new("echo")
        .arg("* 1 * * * test")
        .output()
        .expect("failed to execute child");

    let output = child.stdout;

    println!("{}", String::from_utf8_lossy(&output))
}

fn main() {
    let matches = App::new("Easy Cron")
        .version("0.1.0")
        .author("Sanskar Jethi <sansyrox@gmail.com>")
        .about("Easily schedule your task")
        .arg(
            Arg::new("second")
                .short('s')
                .long("second")
                .takes_value(true)
                .about("Execute the script every nth second"),
        )
        .arg(
            Arg::new("minute")
                .short('m')
                .long("minute")
                .takes_value(true)
                .about("Execute the script every nth minute"),
        )
        .arg(
            Arg::new("hour")
                .long("hour")
                .takes_value(true)
                .about("Execute the script every nth hour"),
        )
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .takes_value(true)
                .about("Execute the script every nth day"),
        )
        .arg(
            Arg::new("month")
                .short('m')
                .long("month")
                .takes_value(true)
                .about("Execute the script every nth month"),
        )
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .takes_value(true)
                .about("Execute the script every nth year"),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .takes_value(false)
                .about("Shows all the current jobs"),
        )
        .get_matches();

    let second = matches.value_of("second").unwrap_or("*");
    let hour = matches.value_of("hour").unwrap_or("*");
    let minute = matches.value_of("minute").unwrap_or("*");
    let day = matches.value_of("day").unwrap_or("*");
    let month = matches.value_of("month").unwrap_or("*");
    let year = matches.value_of("year").unwrap_or("*");

    process_cron()
}
