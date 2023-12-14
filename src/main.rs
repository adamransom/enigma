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
use rotor::{Rotor, RotorKind};

use clap::Parser;
use itertools::iproduct;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Run(args) => run(args),
        cli::Commands::Crack(args) => crack(args),
    }
}

fn run(args: cli::RunArgs) {
    let input = args.input.to_ascii_uppercase().replace('\n', "");

    let reflector = match args.reflector {
        ReflectorKind::A => reflector::A,
        ReflectorKind::B => reflector::B,
        ReflectorKind::C => reflector::C,
    };
    let left_rotor = Rotor::new(args.rotors[0], (args.rings[0] - 1) as usize, args.key[0]);
    let middle_rotor = Rotor::new(args.rotors[1], (args.rings[1] - 1) as usize, args.key[1]);
    let right_rotor = Rotor::new(args.rotors[2], (args.rings[2] - 1) as usize, args.key[2]);

    let mut enigma = Enigma {
        reflector,
        left_rotor,
        right_rotor,
        middle_rotor,
        plugboard: Plugboard::new(&args.plugs),
    };

    println!("{}", enigma.run(&input));
}

fn crack(args: cli::CrackArgs) {
    let input: Vec<char> = args
        .input
        .to_ascii_uppercase()
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect();
    let output: Vec<char> = args
        .output
        .to_ascii_uppercase()
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect();

    let all_rotors = [
        RotorKind::I,
        RotorKind::II,
        RotorKind::III,
        RotorKind::IV,
        RotorKind::V,
    ];

    all_rotors
        .iter()
        .permutations(3)
        .collect::<Vec<Vec<&RotorKind>>>()
        .par_iter()
        .for_each(|rotors| {
            let left_positions = 0..26;
            let middle_positions = 0..26;
            let right_positions = 0..26;
            let middle_ring_settings = 0..26;
            let right_ring_settings = 0..26;

            for (
                left_position,
                middle_position,
                right_position,
                middle_ring_setting,
                right_ring_setting,
            ) in iproduct!(
                left_positions,
                middle_positions,
                right_positions,
                middle_ring_settings,
                right_ring_settings
            ) {
                let left_rotor = Rotor::new(*rotors[0], 0, left_position);
                let middle_rotor = Rotor::new(*rotors[1], middle_ring_setting, middle_position);
                let right_rotor = Rotor::new(*rotors[2], right_ring_setting, right_position);

                let mut enigma = Enigma {
                    reflector: reflector::B,
                    left_rotor,
                    right_rotor,
                    middle_rotor,
                    plugboard: Plugboard::new(&args.plugs),
                };

                if input
                    .iter()
                    .enumerate()
                    .all(|(idx, &c)| output[idx] == enigma.output_for(c))
                {
                    println!(
                        "{:?}({}, 0) {:?}({}, {}) {:?}({}, {})",
                        rotors[0],
                        left_position,
                        rotors[1],
                        middle_position,
                        middle_ring_setting,
                        rotors[2],
                        right_position,
                        right_ring_setting
                    );
                }
            }
        });
}
