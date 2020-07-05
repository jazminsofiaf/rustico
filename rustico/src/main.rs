extern crate clap;
use clap::App;

use rustico::card::french_card::get_card_dec;



use rustico::players::player::Player;
use rustico::players::coordinator::Coordinator;


fn main(){
    let mut amount_of_players: i32 = 4;
    let mut debug: bool = true;
    parse_args(&mut amount_of_players, &mut debug);

    println!("{}", amount_of_players);
    println!("{}", debug);

    let coordinator : Coordinator = Coordinator::new(amount_of_players);
    coordinator.play_game();

}

fn parse_args(_cant_jugadores: &mut i32, _debug: &mut bool) {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let cant_jugadores = matches.value_of("cant_jugadores").unwrap_or("4").parse::<i32>().unwrap();

    let debug = matches.value_of("debug").unwrap_or("true").parse::<bool>().unwrap();

    *_cant_jugadores = cant_jugadores;
    *_debug = debug;

}
