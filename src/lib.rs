const CHARS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

mod helper {
    pub(crate) fn calculate_shift(i: usize) -> i64 {
        if i % 2 == 0 {
            5
        } else if i % 03 == 0 {
            1
        } else if i % 05 == 0 {
            -4
        } else if i % 07 == 0 {
            6
        } else if i % 11 == 0 {
            -2
        } else if i % 13 == 0 {
            -10
        } else {
            i as i64
        }
    }

    pub(crate) fn calculate_index(position: usize, shift: i64) -> usize {
        let index = (position as i64) + shift;
        index.rem_euclid(crate::CHARS.len() as i64) as usize
    }
}

use helper::*;

enum Direction {
    Encode,
    Decode,
}

fn code(input: String, direction: Direction) -> String {
    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if let Some(position) = CHARS.iter().position(|&x| x == c) {
                let shift = calculate_shift(i);
                let index = calculate_index(
                    position,
                    match direction {
                        Direction::Encode => shift,
                        Direction::Decode => -shift,
                    },
                );
                CHARS[index]
            } else {
                c
            }
        })
        .collect()
}

pub fn encode(input: String) -> String {
    code(input, Direction::Encode)
}

pub fn decode(input: String) -> String {
    code(input, Direction::Decode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversible() {
        let input = "Hello, World!".to_string();
        let encoded = encode(input.clone());
        let decoded = decode(encoded);

        assert_eq!(input, decoded);
    }
}
