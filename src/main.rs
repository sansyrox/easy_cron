extern crate daemonize;

use clap::{App, Arg};
use easy_cron::{current_jobs, delete_all_jobs, process_cron};

fn main() {
    let matches = App::new("Easy Cron")
        .version("0.1.1")
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
            Arg::new("command")
                .short('c')
                .long("command")
                .takes_value(true)
                .about("The command to be executed"),
        )
        .arg(
            Arg::new("executable")
                .short('x')
                .long("executable")
                .takes_value(true)
                .about("The path to the executable"),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .takes_value(false)
                .about("Shows all the current jobs"),
        )
        .arg(
            Arg::new("delete_all_jobs")
                .long("delete_all_jobs")
                .takes_value(false)
                .about("Shows all the current jobs"),
        )
        .get_matches();

    let second = matches.value_of("second").unwrap_or("*");
    let hour = matches.value_of("hour").unwrap_or("*");
    let day = matches.value_of("day").unwrap_or("*");
    let month = matches.value_of("month").unwrap_or("*");
    let year = matches.value_of("year").unwrap_or("*");
    let command = matches.value_of("command").unwrap_or("");

    if matches.is_present("jobs") {
        println!("{}", current_jobs());
        return;
    } else if matches.is_present("delete_all_jobs") {
        delete_all_jobs();
        println!("{}", current_jobs());
        return;
    }

    process_cron(&second, &hour, &day, &month, &year, &command)
}
