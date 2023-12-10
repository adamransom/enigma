use crate::reflector::Reflector;
use crate::rotor::Rotor;
use crate::utils::{to_alphabet_index, ALPHA_LENGTH};

/// A collection of all the settings for a specific configurationg of an Enigma machine
pub struct Enigma {
    pub left_rotor: Rotor,
    pub middle_rotor: Rotor,
    pub right_rotor: Rotor,
    pub plugboard: [usize; ALPHA_LENGTH],
    pub reflector: Reflector,
}

impl Enigma {
    /// This takes an input char, then converts it to a index (which can be thought of as the
    /// connection pin on a rotor, reflector or plugboard) and then runs it through the Engima
    /// process, simulating a keypress on the machine.
    pub fn output_for(&mut self, input: char) -> char {
        let mut pin = to_alphabet_index(input as u8);

        pin = self.plugboard[pin];

        // To explain this simply, when a rotor's position matches it's notch value, it causes the
        // rotor next to it to step. The middle rotor has a unique "double stepping" action, in
        // that roughly when it causes the left rotor to step, it also steps. However, due to the
        // intricacies of the actual mechanical setup, in order to match the behaviour perfectly,
        // we don't want to double step if right and middle rotors want to step at the exact same
        // time.
        //
        // - https://www.cryptomuseum.com/people/hamer/files/double_stepping.pdf
        if self.middle_rotor.should_turnover() {
            self.middle_rotor.step();
            self.left_rotor.step();
        } else if self.right_rotor.should_turnover() {
            self.middle_rotor.step();
        }

        self.right_rotor.step();

        pin = self.right_rotor.output_for(pin);
        pin = self.middle_rotor.output_for(pin);
        pin = self.left_rotor.output_for(pin);

        pin = self.reflector.output_for(pin);

        pin = self.left_rotor.input_for(pin);
        pin = self.middle_rotor.input_for(pin);
        pin = self.right_rotor.input_for(pin);

        pin = self.plugboard[pin];

        char::from_u32((pin + 65) as u32).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflector;
    use crate::rotor;

    #[test]
    fn encodes_correctly_with_right_step_only() {
        let mut enigma = Enigma {
            left_rotor: Rotor {
                config: &rotor::I,
                ring_setting: 0,
                position: 0,
            },
            middle_rotor: Rotor {
                config: &rotor::II,
                ring_setting: 0,
                position: 0,
            },
            right_rotor: Rotor {
                config: &rotor::III,
                ring_setting: 0,
                position: 0,
            },
            reflector: reflector::B,
            plugboard: std::array::from_fn(|idx| idx),
        };

        assert_eq!(enigma.output_for('A'), 'B');
        assert_eq!(enigma.output_for('B'), 'J');
        assert_eq!(enigma.output_for('C'), 'E');
    }

    #[test]
    fn encodes_correctly_with_right_and_middle_step() {
        let mut enigma = Enigma {
            left_rotor: Rotor {
                config: &rotor::I,
                ring_setting: 0,
                position: 0,
            },
            middle_rotor: Rotor {
                config: &rotor::II,
                ring_setting: 0,
                position: 0,
            },
            right_rotor: Rotor {
                config: &rotor::III,
                ring_setting: 0,
                position: 19,
            },
            reflector: reflector::B,
            plugboard: std::array::from_fn(|idx| idx),
        };

        assert_eq!(enigma.output_for('A'), 'B');
        assert_eq!(enigma.output_for('B'), 'N');
        assert_eq!(enigma.output_for('C'), 'F');
    }

    #[test]
    fn encodes_correctly_with_double_middle_step() {
        let mut enigma = Enigma {
            left_rotor: Rotor {
                config: &rotor::I,
                ring_setting: 0,
                position: 0,
            },
            middle_rotor: Rotor {
                config: &rotor::II,
                ring_setting: 0,
                position: 3,
            },
            right_rotor: Rotor {
                config: &rotor::III,
                ring_setting: 0,
                position: 21,
            },
            reflector: reflector::B,
            plugboard: std::array::from_fn(|idx| idx),
        };

        assert_eq!(enigma.output_for('A'), 'Q');
        assert_eq!(enigma.output_for('B'), 'W');
        assert_eq!(enigma.output_for('C'), 'K');
    }

    #[test]
    fn encodes_correctly_with_immediate_right_and_middle_step() {
        let mut enigma = Enigma {
            left_rotor: Rotor {
                config: &rotor::I,
                ring_setting: 0,
                position: 0,
            },
            middle_rotor: Rotor {
                config: &rotor::II,
                ring_setting: 0,
                position: 4,
            },
            right_rotor: Rotor {
                config: &rotor::III,
                ring_setting: 0,
                position: 21,
            },
            reflector: reflector::B,
            plugboard: std::array::from_fn(|idx| idx),
        };

        assert_eq!(enigma.output_for('A'), 'G');
        assert_eq!(enigma.output_for('B'), 'W');
        assert_eq!(enigma.output_for('C'), 'K');
    }
}
