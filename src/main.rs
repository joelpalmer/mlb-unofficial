extern crate clap;
use clap::{App, Arg, SubCommand};
mod game;

fn main() {
    let matches = App::new("MLB Scores & Stats")
        .version("1.0")
        .author("Joel Palmer <joel@joel.systems>")
        .about("Only gets Dodgers scores for now")
        .arg(
            Arg::with_name("TEAM")
                .help("Sets the team to look up")
                .required(true)
                .index(1),
        )
        .get_matches();

    println!("Team: {}", matches.value_of("TEAM").unwrap());
}
