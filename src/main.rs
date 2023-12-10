mod cli;
mod enigma;
mod reflector;
mod rotor;
mod utils;

use cli::Cli;
use enigma::Enigma;
use reflector::ReflectorKind;
use rotor::{Rotor, RotorKind};
use utils::ALPHA_LENGTH;

use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let input = cli.input.replace('\n', "");

    let reflector = match cli.reflector {
        ReflectorKind::A => reflector::A,
        ReflectorKind::B => reflector::B,
        ReflectorKind::C => reflector::C,
    };

    let left_rotor = create_rotor(cli.rotors[0], cli.rings[0], cli.key[0]);
    let middle_rotor = create_rotor(cli.rotors[1], cli.rings[1], cli.key[1]);
    let right_rotor = create_rotor(cli.rotors[2], cli.rings[2], cli.key[2]);

    let mut enigma = Enigma {
        reflector,
        left_rotor,
        right_rotor,
        middle_rotor,
        plugboard: create_plugboard(&cli.plugs),
    };

    let mut output = Vec::new();

    input.split_whitespace().for_each(|section| {
        let mut section_output = String::from("");

        for c in section.chars() {
            section_output.push(enigma.output_for(c));
        }

        output.push(section_output);
    });

    println!("{}", output.join(" "));
}

/// Create a rotor from a specific config (i.e. wiring) with a ring setting and initial position
fn create_rotor(rotor: RotorKind, ring_setting: u8, position: usize) -> Rotor {
    let adjusted_ring_setting = (ring_setting - 1) as usize;

    let config = match rotor {
        RotorKind::I => &rotor::I,
        RotorKind::II => &rotor::II,
        RotorKind::III => &rotor::III,
        RotorKind::IV => &rotor::IV,
        RotorKind::V => &rotor::V,
    };

    Rotor {
        config,
        position,
        ring_setting: adjusted_ring_setting,
    }
}

/// Take an array of index pairs (representing the wired connections between two letters on the
/// plugboard) and create an array where the index is the input letter and the value is the
/// output letter e.g. AB CD would map to [1, 0, 3, 2]
fn create_plugboard(pairs: &[(usize, usize)]) -> [usize; ALPHA_LENGTH] {
    let mut plugboard: [usize; ALPHA_LENGTH] = std::array::from_fn(|idx| idx);

    pairs.iter().for_each(|&(left, right)| {
        plugboard[left] = right;
        plugboard[right] = left;
    });

    plugboard
}
