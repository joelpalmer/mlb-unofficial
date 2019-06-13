extern crate chrono;
extern crate clap;
use chrono::offset::LocalResult;
use chrono::prelude::*;
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
                .required(true),
        )
        .arg(
            Arg::with_name("DATE")
                .help("Sets the date to look up: 2019-06-10")
                .required(false)
                .default_value_if("TEAM", None, get_default_date().as_str()),
        )
        .get_matches();

    let game = game::Game::new(
        matches.value_of("TEAM").unwrap(),
        matches.value_of("DATE").unwrap(),
    );

    println!(
        "{} are beating the {}: {} to {}",
        game.home_team, game.away_team, game.home_score, game.away_score
    );
}
fn get_default_date() -> String {
    Local::today().format("%Y-%m-%d").to_string() // format: 2019-06-10
}
