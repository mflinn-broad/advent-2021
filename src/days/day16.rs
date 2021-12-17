use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day16.txt").unwrap();
    let input = process(&raw_input);

    println!("{:?}", input);
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

fn parse_packet(bytes: &[u8]) -> (&[u8], u8) {
    let (rest, curr_vers, packet_type) = parse_header(bytes);
    match packet_type {
        Type::Literal => (parse_literal(rest), curr_vers),
        Type::Operator => todo!()
    }
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
        let rem = parse_literal(&[1,0,1,0,1,1,0,0,1,0,0,0,1,0,1]);
        assert!(rem.is_empty());

        let rem = parse_literal(&[1,0,1,0,1,1,0,0,1,0,0,0,1,0,1,1,0,1]);
        assert_eq!(rem, &[1,0,1]);
    }
}
