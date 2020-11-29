use std::time::SystemTime;

use clap::{App, AppSettings, Arg};

use rsutils::math::fibonacci;

fn main() {
    let matches = App::new("Math")
        .version("0.1.0")
        .author("LonelyPale")
        .about("Rust Math Utils.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("fibonacci")
                .alias("fi")
                .short_flag('F')
                .long_flag("fi")
                .version("0.1")
                .about("Fibonacci sequence")
                .arg(
                    Arg::new("NUMBER")
                        .about("Index number")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("recursive")
                        .short('r')
                        .long("recursive")
                        .about("Use recursive algorithm"),
                )
                .arg(
                    Arg::new("list")
                        .short('l')
                        .long("list")
                        .about("Display sequence"),
                )
                .arg(Arg::new("time").short('t').long("time").about("Run time")),
        )
        .get_matches();

    // match matches.subcommand_name() {
    //     Some("fibonacci") => println!(""),
    //     None => println!("No subcommand was used"),
    //     _ => println!("Some other subcommand was used"),
    // }

    if let Some(ref matches) = matches.subcommand_matches("fibonacci") {
        let number: i32 = matches
            .value_of("NUMBER")
            .unwrap()
            .parse()
            .expect("Not a number!");

        let start = SystemTime::now();

        if matches.is_present("list") {
            fibonacci::sequence(number);
        } else {
            let fi_num;
            if matches.is_present("recursive") {
                fi_num = fibonacci::recursive(number);
            } else {
                fi_num = fibonacci::non_recursive(number);
            }
            println!("fibonacci({})={}", number, fi_num);
        }

        if matches.is_present("time") {
            println!("{:?}", start.elapsed());
        }
    } else {
        unreachable!();
    }
}
