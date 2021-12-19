use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day16.txt").unwrap();
    let input = process(&raw_input);

    println!("Result: {:?}", parse(&input));
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
        t => (&byte[3..], Type::Operator(t)),
    }
}

fn parse_header(bytes: &[u8]) -> (&[u8], u8, Type) {
    let (remaining, vers) = parse_vers(bytes);
    let (remaining, type_id) = parse_type(remaining);
    (remaining, vers, type_id)
}

fn parse_literal(mut bytes: &[u8]) -> (&[u8], u64) {
    let mut literal_bits = vec![];
    (0..)
        .take_while(|_| {
            let continues = bytes[0] == 1;
            literal_bits.extend(&bytes[1..5]);
            bytes = &bytes[5..];
            continues
        })
        .count();

    (bytes, bits_to_num(&literal_bits))
}

fn parse_operator(mut bytes: &[u8], operator_type: u8) -> (&[u8], u64) {
    let length_type = bytes[0];
    bytes = &bytes[1..];

    let mut sub_packet_values: Vec<u64> = vec![];

    if length_type == 0 {
        let l = bits_to_num(&bytes[0..15]) as usize;
        bytes = &bytes[15..];
        let rest = &bytes[l..];

        let mut sub_packets = &bytes[0..l];
        while !sub_packets.is_empty() {
            let (new_sub_packets, val) = parse_packet(sub_packets);
            sub_packet_values.push(val);
            sub_packets = new_sub_packets;
        }
        bytes = rest;
    } else {
        let desired_sub_packet_count = bits_to_num(&bytes[0..11]);
        bytes = &bytes[11..];
        (0..desired_sub_packet_count).for_each(|_| {
            let (rem, val) = parse_packet(bytes);
            sub_packet_values.push(val);
            bytes = rem;
        });
    }

    match operator_type {
        0 => (bytes, sub_packet_values.iter().sum()),
        1 => (bytes, sub_packet_values.iter().product()),
        2 => (bytes, *sub_packet_values.iter().min().unwrap()),
        3 => (bytes, *sub_packet_values.iter().max().unwrap()),
        5 => (bytes, (sub_packet_values[0] > sub_packet_values[1]) as u64),
        6 => (bytes, (sub_packet_values[0] < sub_packet_values[1]) as u64),
        7 => (bytes, (sub_packet_values[0] == sub_packet_values[1]) as u64),
        _ => panic!("unreachable"),
    }
}

fn parse_packet(bytes: &[u8]) -> (&[u8], u64) {
    let (rest, _, packet_type) = parse_header(bytes);

    match packet_type {
        Type::Literal => parse_literal(rest),
        Type::Operator(t) => parse_operator(rest, t),
    }
}

fn bits_to_num(bits: &[u8]) -> u64 {
    bits.iter().fold(0, |res, &bit| (res << 1) ^ bit as u64)
}

fn parse(input: &[u8]) -> u64 {
    parse_packet(input).1
}

#[derive(Debug, PartialEq, Eq)]
enum Type {
    Literal,
    Operator(u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_computation(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day16.txt").unwrap();
        b.iter(|| {
            let input = process(&raw_input);
            parse(&input);
        })
        
    }

    #[test]
    fn test_part_2() {
        let input = "C200B40A82";
        let input = process(input);
        let result = parse(&input);
        assert_eq!(result, 3)
    }
}
