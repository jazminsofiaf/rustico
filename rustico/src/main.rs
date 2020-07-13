extern crate clap;

use clap::App;
use colored::*;

use rustico::players::coordinator::Coordinator;
use rustico::logger::logger::Logger;
use std::sync::mpsc;
use rustico::consts::consts::consts::STOP_LOGGING_MSG;

fn main() {
    let mut number_of_players: i32 = 4;
    let mut debug: bool = true;
    parse_args(&mut number_of_players, &mut debug);
    display_game_info(&mut number_of_players, &mut debug);

    let (logger_sender, logger_receiver) = mpsc::channel();
    let logger = Logger::new(debug, logger_receiver);


    let mut coordinator: Coordinator = Coordinator::new(number_of_players, logger_sender.clone());
    coordinator.let_the_game_begin();

    /* send end of game msg to logger */
    logger_sender.send(STOP_LOGGING_MSG.to_string()).expect("error sending msg2");
    logger.join().expect("error joining logger");
}

fn parse_args(_cant_jugadores: &mut i32, _debug: &mut bool) {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let cant_jugadores = matches.value_of("cant_jugadores").unwrap_or("4").parse::<i32>().unwrap();

    let debug = matches.value_of("debug").unwrap_or("true").parse::<bool>().unwrap();

    *_cant_jugadores = cant_jugadores;
    *_debug = debug;
}

fn display_game_info(number_of_players: &mut i32, debug: &mut bool) {
    println!("{}", format!("\n
ooooooooo.   ooooo     ooo  .oooooo..o ooooooooooooo ooooo   .oooooo.     .oooooo.
`888   `Y88. `888'     `8' d8P'    `Y8 8'   888   `8 `888'  d8P'  `Y8b   d8P'  `Y8b
 888   .d88'  888       8  Y88bo.           888       888  888          888      888
 888ooo88P'   888       8   `\"Y8888o.       888       888  888          888      888
 888`88b.     888       8       `\"Y88b      888       888  888          888      888
 888  `88b.   `88.    .8'  oo     .d8P      888       888  `88b    ooo  `88b    d88'
o888o  o888o    `YbodP'    8\"\"88888P'      o888o     o888o  `Y8bood8P'   `Y8bood8P'
"));

    println!("{}", format!("⚓  GAME LOADING...").bright_white());
    println!("{}", format!("ℹ️  INFO:").bright_white());
    println!("{}", format!("\t- debug: {}", debug).bright_white());
    print!("{}", format!("\t- number of players: ").bright_white());
    for _ in 0..*number_of_players {
        print!("{}", format!(" 🖐️ ").bright_white());
    }
    println!("{}", format!("\nTo change any of the settings, please run the game with the\n\
                           --help (-h) flag for more info on how to do it.\n").white().dimmed());
}
