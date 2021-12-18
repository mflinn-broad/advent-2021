use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day16.txt").unwrap();
    let input = process(&raw_input);

    println!("Part 1: {:?}", part_1(&input));
}

fn process(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .flat_map(|hex| {
            let hex_code = hex.to_digit(16).unwrap();
            (0..4).map(move |pos| (1 & hex_code >> (3 - pos)) as u8)
        })
        .collect()
}

fn parse_vers(byte: &[u8]) -> (&[u8], u8) {
    let vers = byte
        .iter()
        .take(3)
        .fold(0, |ver_num, bit| (ver_num << 1) ^ bit);

    (&byte[3..], vers)
}

fn parse_type(byte: &[u8]) -> (&[u8], Type) {
    let type_id = byte.iter().take(3).fold(0, |id, bit| (id << 1) ^ bit);

    match type_id {
        4 => (&byte[3..], Type::Literal),
        _ => (&byte[3..], Type::Operator),
    }
}

fn parse_header(bytes: &[u8]) -> (&[u8], u8, Type) {
    let (remaining, vers) = parse_vers(bytes);
    let (remaining, type_id) = parse_type(remaining);
    (remaining, vers, type_id)
}

fn parse_literal(mut bytes: &[u8]) -> &[u8] {
    (0..)
        .take_while(|_| {
            let continues = bytes[0] == 1;
            bytes = &bytes[5..];
            continues
        })
        .count();

    bytes
}

fn parse_operator(mut bytes: &[u8]) -> (&[u8], u64) {
    let length_type = bytes[0];
    bytes = &bytes[1..];

    match length_type {
        0 => {
            let l = bytes
                .iter()
                .take(15)
                .fold(0, |l, &bit| ((l as usize) << 1) ^ bit as usize);
            bytes = &bytes[15..];
            let rest = &bytes[l..];
            let mut sub_packets = &bytes[0..l];
            let mut sub_packet_version_count = 0;
            while !sub_packets.is_empty() {
                let (new_sub_packets, vers) = parse_packet(sub_packets);
                sub_packet_version_count += vers;
                sub_packets = new_sub_packets;
            }
            (rest, sub_packet_version_count)
        }
        1 => {
            let desired_sub_packet_count = bytes
                .iter()
                .take(11)
                .fold(0, |l, &bit| ((l as u32) << 1) ^ bit as u32);
            bytes = &bytes[11..];
            let sub_packet_versions = (0..desired_sub_packet_count)
                .map(|_| {
                    let (rem, vers) = parse_packet(bytes);
                    bytes = rem;
                    vers
                })
                .sum::<u64>();

            (bytes, sub_packet_versions)
        }
        _ => panic!("unreachable"),
    }
}

fn parse_packet(bytes: &[u8]) -> (&[u8], u64) {
    let (rest, curr_vers, packet_type) = parse_header(bytes);
    match packet_type {
        Type::Literal => (parse_literal(rest), curr_vers as u64),
        Type::Operator => {
            let (rest, sub_packet_versions) = parse_operator(rest);
            (rest, curr_vers as u64 + sub_packet_versions)
        }
    }
}

fn part_1(mut input: &[u8]) -> u64 {
    let mut vers_sum = 0;
    while input.len() > 6 {
        let res = parse_packet(input);
        input = res.0;
        vers_sum += res.1;
    }

    vers_sum
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Literal,
    Operator,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ver() {
        let (rem, vers) = parse_vers(&[1, 1, 0]);
        assert_eq!(vers, 6);
        assert!(rem.is_empty());

        let (rem, vers) = parse_vers(&[1, 0, 1, 1, 0]);
        assert_eq!(vers, 5);
        assert_eq!(rem, &[1, 0]);
    }

    #[test]
    fn test_parse_type() {
        let (rem, t) = parse_type(&[1, 0, 0]);
        assert_eq!(t, Type::Literal);
        assert!(rem.is_empty());

        let (rem, t) = parse_type(&[1, 0, 1, 0, 1, 0]);
        assert_eq!(t, Type::Operator);
        assert_eq!(rem, &[0, 1, 0]);
    }

    #[test]
    fn test_parse_header() {
        let (rem, vers, t) = parse_header(&[1, 1, 0, 1, 0, 0]);
        assert_eq!(vers, 6);
        assert_eq!(t, Type::Literal);
        assert!(rem.is_empty());

        let (rem, vers, t) = parse_header(&[1, 0, 1, 0, 0, 1, 1, 0, 0, 1]);
        assert_eq!(vers, 5);
        assert_eq!(t, Type::Operator);
        assert_eq!(rem, &[1, 0, 0, 1]);
    }

    #[test]
    fn test_parse_literal() {
        let rem = parse_literal(&[1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1]);
        assert!(rem.is_empty());

        let rem = parse_literal(&[1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1]);
        assert_eq!(rem, &[1, 0, 1]);
    }

    #[test]
    fn test_part_1() {
        let input = "8A004A801A8002F478";
        let input = process(input);
        let res = part_1(&input);
        assert_eq!(res, 16);

    }
}
