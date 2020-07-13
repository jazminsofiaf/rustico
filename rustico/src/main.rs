extern crate clap;

use clap::App;
use colored::*;

use rustico::players::coordinator::Coordinator;

fn main() {
    let mut number_of_players: i32 = 4;
    let mut debug: bool = true;
    parse_args(&mut number_of_players, &mut debug);

    println!("{}", format!("GAME LOADING...\nINFO:").bright_white());
    println!("{}", format!("\t- debug: {}", debug).bright_white());
    println!("{}", format!("\t- number of players: {}", number_of_players).bright_white());
    println!("{}", format!("\nTo change any of the settings, please run the game with the\n\
                           --help (-h) flag for more info on how to do it.\n").white().dimmed());


    let mut coordinator: Coordinator = Coordinator::new(number_of_players);
    coordinator.let_the_game_begin();
}

fn parse_args(_cant_jugadores: &mut i32, _debug: &mut bool) {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let cant_jugadores = matches.value_of("cant_jugadores").unwrap_or("4").parse::<i32>().unwrap();

    let debug = matches.value_of("debug").unwrap_or("true").parse::<bool>().unwrap();

    *_cant_jugadores = cant_jugadores;
    *_debug = debug;
}
