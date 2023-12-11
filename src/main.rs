mod cli;
mod enigma;
mod plugboard;
mod reflector;
mod rotor;
mod utils;

use cli::Cli;
use enigma::Enigma;
use plugboard::Plugboard;
use reflector::ReflectorKind;
use rotor::Rotor;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let input = cli.input.to_ascii_uppercase().replace('\n', "");

    let reflector = match cli.reflector {
        ReflectorKind::A => reflector::A,
        ReflectorKind::B => reflector::B,
        ReflectorKind::C => reflector::C,
    };

    let left_rotor = Rotor::new(cli.rotors[0], (cli.rings[0] - 1) as usize, cli.key[0]);
    let middle_rotor = Rotor::new(cli.rotors[1], (cli.rings[1] - 1) as usize, cli.key[1]);
    let right_rotor = Rotor::new(cli.rotors[2], (cli.rings[2] - 1) as usize, cli.key[2]);

    let mut enigma = Enigma {
        reflector,
        left_rotor,
        right_rotor,
        middle_rotor,
        plugboard: Plugboard::new(&cli.plugs),
    };

    println!("{}", enigma.run(&input));
}
