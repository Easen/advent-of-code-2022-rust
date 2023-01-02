use std::{collections::HashSet, ops::AddAssign};

fn decode_datastream(datastream: &str, marker_len: usize) -> Option<usize> {
    if datastream.len() < marker_len {
        return None;
    }

    let mut current_pos = marker_len;
    let mut marker_buffer: HashSet<char> = HashSet::new();
    while current_pos <= datastream.len() {
        marker_buffer.clear();
        let range = &datastream[current_pos - marker_len..current_pos];
        range.chars().for_each(|c| {
            marker_buffer.insert(c);
        });
        if marker_buffer.len() == marker_len {
            return Some(current_pos);
        }
        current_pos.add_assign(1);
    }
    None
}
fn main() {
    let input = include_str!("../../inputs/6.txt");

    if let Some(marker) = decode_datastream(input, 4) {
        println!("part1: {}", marker);
    }

    if let Some(marker) = decode_datastream(input, 14) {
        println!("part2: {}", marker);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_less_than_4_chars_to_be_none() {
        assert_eq!(decode_datastream("", 4), None);
        assert_eq!(decode_datastream("1", 4), None);
        assert_eq!(decode_datastream("12", 4), None);
        assert_eq!(decode_datastream("123", 4), None);
    }

    #[test]
    fn test_4_chars() {
        assert_eq!(decode_datastream("1234", 4), Some(4));
    }

    #[test]
    fn test_part1_example_1() {
        assert_eq!(
            decode_datastream("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4),
            Some(7)
        );
    }
    #[test]
    fn test_part1_example_2() {
        assert_eq!(
            decode_datastream("bvwbjplbgvbhsrlpgdmjqwftvncz", 4),
            Some(5)
        );
    }
    #[test]
    fn test_part1_example_3() {
        assert_eq!(
            decode_datastream("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            Some(10)
        );
    }
    #[test]
    fn test_part1_example_4() {
        assert_eq!(
            decode_datastream("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
            Some(11)
        );
    }
    #[test]
    fn test_part1_example_5() {
        assert_eq!(
            decode_datastream("nppdvjthqldpwncqszvftbrmjlhg", 4),
            Some(6)
        );
    }

    #[test]
    fn test_part2_example_1() {
        assert_eq!(
            decode_datastream("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14),
            Some(19)
        );
    }
    #[test]
    fn test_part2_example_2() {
        assert_eq!(
            decode_datastream("bvwbjplbgvbhsrlpgdmjqwftvncz", 14),
            Some(23)
        );
    }
    #[test]
    fn test_part2_example_3() {
        assert_eq!(
            decode_datastream("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            Some(29)
        );
    }
    #[test]
    fn test_part2_example_4() {
        assert_eq!(
            decode_datastream("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            Some(26)
        );
    }
    #[test]
    fn test_part2_example_5() {
        assert_eq!(
            decode_datastream("nppdvjthqldpwncqszvftbrmjlhg", 14),
            Some(23)
        );
    }
}
