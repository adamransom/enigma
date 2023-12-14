use crate::utils::{to_alphabet_index, ALPHA_LENGTH};

use clap::ValueEnum;
use const_for::const_for;

// https://en.wikipedia.org/wiki/Enigma_rotor_details
pub const I: RotorConfig = RotorConfig::new(RotorKind::I, b"EKMFLGDQVZNTOWYHXUSPAIBRCJ", b'Q');
pub const II: RotorConfig = RotorConfig::new(RotorKind::II, b"AJDKSIRUXBLHWTMCQGZNPYFVOE", b'E');
pub const III: RotorConfig = RotorConfig::new(RotorKind::III, b"BDFHJLCPRTXVZNYEIWGAKMUSQO", b'V');
pub const IV: RotorConfig = RotorConfig::new(RotorKind::IV, b"ESOVPZJAYQUIRHXLNFTGKDCMWB", b'J');
pub const V: RotorConfig = RotorConfig::new(RotorKind::V, b"VZBRGITYUPSDNHLXAWMJQOFECK", b'Z');

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[value(rename_all = "UPPER")]
pub enum RotorKind {
    I,
    II,
    III,
    IV,
    V,
}

/// Each rotor has a specific wiring pattern i.e. on the I rotor, A is mapped to E, but on the II
/// rotor, A is mapped to itself. This struct holds the wire mapping, an inverse of that mapping
/// and the specific point at which this rotor will cause the next rotor to step.
///
/// We store the inverse mapping as in the Enigma, the current flows through the rotors one way,
/// into the reflector and then back through the rotors in reverse and having both mappings makes
/// the operation quicker.
#[derive(Debug)]
pub struct RotorConfig {
    _kind: RotorKind,
    wiring: [usize; ALPHA_LENGTH],
    inverse: [usize; ALPHA_LENGTH],
    turnover: usize,
}

impl RotorConfig {
    /// Basically sets up the wiring array from a string of uppercase alphabet characters e.g a
    /// string of "DBAC" would map to [3, 1, 0, 2], which means "for an input A (i.e. index 0), output
    /// D (i.e. value 3)". The inverse wiring for this would be [2, 1, 3, 0].
    pub const fn new(kind: RotorKind, mapping: &[u8; ALPHA_LENGTH], turnover: u8) -> Self {
        let mut wiring = [0; ALPHA_LENGTH];
        let mut inverse = [0; ALPHA_LENGTH];

        const_for!(input_pin in 0..ALPHA_LENGTH => {
            let output_pin = to_alphabet_index(mapping[input_pin]);

            wiring[input_pin] = output_pin;
            inverse[output_pin] = input_pin;
        });

        Self {
            _kind: kind,
            wiring,
            inverse,
            turnover: to_alphabet_index(turnover),
        }
    }
}

#[derive(Debug)]
pub struct Rotor {
    pub config: &'static RotorConfig,
    pub position: usize,
    pub ring_setting: usize,
}

/// Add two numbers, wrapping round to 0 if they exceed 25
fn wrapping_add(a: usize, b: usize) -> usize {
    match a + b {
        val @ 0..=25 => val,
        val => val - ALPHA_LENGTH,
    }
}

/// Subtract two numbers, wrapping round to 25 if they go below 0
fn wrapping_sub(a: usize, b: usize) -> usize {
    match a.overflowing_sub(b) {
        (_, true) => ALPHA_LENGTH - b + a,
        (val, false) => val,
    }
}

impl Rotor {
    /// Create a rotor from a specific config (i.e. wiring) with a ring setting and initial position
    pub fn new(rotor: RotorKind, ring_setting: usize, position: usize) -> Self {
        let config = match rotor {
            RotorKind::I => &I,
            RotorKind::II => &II,
            RotorKind::III => &III,
            RotorKind::IV => &IV,
            RotorKind::V => &V,
        };

        Rotor {
            config,
            position,
            ring_setting,
        }
    }

    pub fn step(&mut self) {
        self.position = wrapping_add(self.position, 1);
    }

    /// Get the output position for a specific input position, if going right to left on the rotor.
    pub fn output_for(&self, input_position: usize) -> usize {
        self.output_position(&self.config.wiring, input_position)
    }

    /// Get the output position for a specific input position, if going left to right on the rotor
    /// (i.e. in reverse, after the reflector).
    pub fn input_for(&self, output_position: usize) -> usize {
        self.output_position(&self.config.inverse, output_position)
    }

    pub fn should_turnover(&self) -> bool {
        self.position == self.config.turnover
    }

    /// Take a specific input position (say A or 0) and calculate the output position relative to
    /// other rotors (i.e. at what position it will enter the next rotor), taking into account the
    /// ring setting and position (rotation) of the rotor.
    fn output_position(&self, wiring: &[usize; ALPHA_LENGTH], input_position: usize) -> usize {
        let offset = wrapping_sub(self.position, self.ring_setting);

        let input_pin = wrapping_add(input_position, offset);

        let output_pin = wiring[input_pin];

        wrapping_sub(output_pin, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_creates_correct_wiring() {
        let config = RotorConfig::new(RotorKind::I, b"EKMFLGDQVZNTOWYHXUSPAIBRCJ", b'Q');

        assert_eq!(
            config.wiring,
            [
                4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1,
                17, 2, 9
            ]
        );
    }

    #[test]
    fn config_creates_correct_inverse_wiring() {
        let config = RotorConfig::new(RotorKind::I, b"EKMFLGDQVZNTOWYHXUSPAIBRCJ", b'Q');

        assert_eq!(
            config.inverse,
            [
                20, 22, 24, 6, 0, 3, 5, 15, 21, 25, 1, 4, 2, 10, 12, 19, 7, 23, 18, 11, 17, 8, 13,
                16, 14, 9
            ]
        );
    }

    #[test]
    fn it_gives_output_as_input_to_next_rotor() {
        let rotor = Rotor {
            config: &I,
            ring_setting: 0,
            position: 0,
        };

        assert_eq!(rotor.output_for(0), 4);
    }

    #[test]
    fn it_gives_inverse_output_as_input_to_next_rotor() {
        let rotor = Rotor {
            config: &I,
            ring_setting: 0,
            position: 0,
        };

        assert_eq!(rotor.input_for(4), 0);
    }

    #[test]
    fn it_handles_changing_position() {
        let rotor = Rotor {
            config: &I,
            ring_setting: 0,
            position: 1,
        };

        assert_eq!(rotor.output_for(0), 9);
        assert_eq!(rotor.input_for(9), 0);
    }

    #[test]
    fn it_handles_changing_ring_setting() {
        let rotor = Rotor {
            config: &I,
            ring_setting: 1,
            position: 0,
        };

        assert_eq!(rotor.output_for(0), 10);
        assert_eq!(rotor.input_for(10), 0);
    }

    #[test]
    fn it_handles_changing_position_and_ring() {
        let rotor = Rotor {
            config: &I,
            ring_setting: 1,
            position: 1,
        };

        assert_eq!(rotor.output_for(0), 4);
        assert_eq!(rotor.input_for(4), 0);
    }
}
