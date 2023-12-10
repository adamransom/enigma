use crate::utils::{to_alphabet_index, ALPHA_LENGTH};

use clap::ValueEnum;
use const_for::const_for;

pub const A: Reflector = Reflector::new(ReflectorKind::A, b"EJMZALYXVBWFCRQUONTSPIKHGD");
pub const B: Reflector = Reflector::new(ReflectorKind::B, b"YRUHQSLDPXNGOKMIEBFZCWVJAT");
pub const C: Reflector = Reflector::new(ReflectorKind::C, b"FVPJIAOYEDRZXWGCTKUQSBNMHL");

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "UPPER")]
pub enum ReflectorKind {
    A,
    B,
    C,
}

/// The reflector (also know as Umkehrwalze or UKW) simply takes an input from the leftmost rotor
/// and maps it to a different output to be fed back into the leftmost rotor (and onwards) in the
/// reverse direction.
///
/// - https://en.wikipedia.org/wiki/Enigma_rotor_details
/// - http://www.ellsbury.com/enigma2.htm
pub struct Reflector {
    _kind: ReflectorKind,
    wiring: [usize; ALPHA_LENGTH],
}

impl Reflector {
    /// Basically sets up the wiring array from a string of uppercase alphabet characters e.g a
    /// string of "CBA" would map to [2, 1, 0], which means "for an input A (i.e. index 0), output
    /// C (i.e. value 2)"
    pub const fn new(kind: ReflectorKind, mapping: &[u8; ALPHA_LENGTH]) -> Self {
        let mut wiring = [0; ALPHA_LENGTH];

        const_for!(input_pin in 0..ALPHA_LENGTH => {
            let output_pin = to_alphabet_index(mapping[input_pin]);
            wiring[input_pin] = output_pin;
        });

        Self {
            _kind: kind,
            wiring,
        }
    }

    /// Get the output position for a specific input position
    pub fn output_for(&self, input_position: usize) -> usize {
        self.wiring[input_position]
    }
}
