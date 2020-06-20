extern crate clap;
use clap::App;

fn main(){
    let mut cant_jugadores: u8 = 4;
    let mut debug: bool = true;
    parse_args(&mut cant_jugadores, &mut debug);

    println!("{}", cant_jugadores);
    println!("{}", debug);
}

fn parse_args(_cant_jugadores: &mut u8, _debug: &mut bool) {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let cant_jugadores = matches.value_of("cant_jugadores").unwrap_or("4").parse::<u8>().unwrap();

    let debug = matches.value_of("debug").unwrap_or("true").parse::<bool>().unwrap();

    *_cant_jugadores = cant_jugadores;
    *_debug = debug;

}
