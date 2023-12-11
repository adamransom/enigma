use crate::utils::ALPHA_LENGTH;

/// The plugboard (also know as Steckerbrett) permitted variable wiring between pairs of letters
/// that could be configured by the operator. In theory, 13 sets of wires could be used to pair up
/// all 26 letters, however only 10 were really ever used.
pub struct Plugboard {
    wiring: [usize; ALPHA_LENGTH],
}

impl Plugboard {
    /// Take an array of index pairs (representing the wired connections between two letters on the
    /// plugboard) and create an array where the index is the input letter and the value is the
    /// output letter e.g. AB CD would map to [1, 0, 3, 2]
    pub fn new(pairs: &[(usize, usize)]) -> Self {
        let mut wiring: [usize; ALPHA_LENGTH] = std::array::from_fn(|idx| idx);

        pairs.iter().for_each(|&(left, right)| {
            wiring[left] = right;
            wiring[right] = left;
        });

        Self { wiring }
    }

    /// Get the output position for a specific input position
    pub fn output_for(&self, input_position: usize) -> usize {
        self.wiring[input_position]
    }
}
