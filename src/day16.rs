use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day16)]
pub fn generate(inp: &str) -> Option<String> {
    inp.lines().next().map(ToString::to_string)
}

#[derive(Clone, Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    value: Option<u64>,
    sub_packets: Vec<Packet>,
}

fn from_binary(inp: &str) -> u64 {
    u64::from_str_radix(inp, 2).expect("Input needs to be a binary string")
}

fn parse_hex_packet(packet: &str) -> Packet {
    let mut as_binary = packet
        .chars()
        .filter_map(|it| it.to_digit(16).map(|d| format!("{d:04b}")))
        .flat_map(|it| it.chars().collect_vec())
        .join("");
    parse_packet(&mut as_binary)
}

fn consume_first_n(inp: &mut String, n: usize) -> String {
    inp.drain(..n).collect::<String>()
}

fn parse_packet_version(input: &mut String) -> u64 {
    let version_string = consume_first_n(input, 3);
    from_binary(&version_string)
}

fn parse_type_id_version(input: &mut String) -> u64 {
    let type_id_string = consume_first_n(input, 3);
    from_binary(&type_id_string)
}

fn parse_sub_packet_length(input: &mut String) -> u64 {
    let bits_as_str = consume_first_n(input, 15);
    from_binary(&bits_as_str)
}

fn parse_literal_value_packet(version: u64, type_id: u64, input: &mut String) -> Packet {
    // literal value
    let mut number = String::new();

    loop {
        let chunk = consume_first_n(input, 5);
        number.push_str(&chunk[1..]);

        if chunk.starts_with('0') {
            break;
        }
    }

    let val = from_binary(&number);

    Packet {
        version,
        type_id,
        value: Some(val),
        sub_packets: Vec::new(),
    }
}

#[allow(clippy::cast_possible_truncation)]
fn parse_sub_packets_by_length(input: &mut String) -> Vec<Packet> {
    let mut result = Vec::new();

    // next 15 bits => total length of sub-packets
    let bits = parse_sub_packet_length(input) as usize;
    let mut sub_packets = consume_first_n(input, bits);

    while !sub_packets.is_empty() {
        let sub_pack = parse_packet(&mut sub_packets);
        result.push(sub_pack);
    }

    result
}

fn parse_sub_packets_by_count(input: &mut String) -> Vec<Packet> {
    let mut result = Vec::new();

    // next 11 bits => number of sub-packets
    let length_as_str = consume_first_n(input, 11);
    let num_sub_packets = from_binary(&length_as_str);

    for _ in 0..num_sub_packets {
        let p = parse_packet(input);
        result.push(p);
    }

    result
}

fn parse_packet(input: &mut String) -> Packet {
    let packet_version = parse_packet_version(input);
    let packet_type_id = parse_type_id_version(input);

    if packet_type_id == 4 {
        return parse_literal_value_packet(packet_version, packet_type_id, input);
    }

    // operator packet
    let length_type_id = consume_first_n(input, 1);
    let sub_packets = if length_type_id == "0" {
        parse_sub_packets_by_length(input)
    } else if length_type_id == "1" {
        parse_sub_packets_by_count(input)
    } else {
        panic!("Invalid length type id: {length_type_id}")
    };

    Packet {
        version: packet_version,
        type_id: packet_type_id,
        value: None,
        sub_packets,
    }
}

fn packet_version_sum(packet: &Packet) -> u64 {
    let mut res = packet.version;
    for p in &packet.sub_packets {
        res += packet_version_sum(p);
    }

    res
}

#[aoc(day16, part1)]
pub fn part1(inp: &str) -> u64 {
    let packet = parse_hex_packet(inp);
    packet_version_sum(&packet)
}

fn do_calc(packet: &Packet) -> u64 {
    if let Some(val) = packet.value {
        assert!(packet.sub_packets.is_empty());
        return val;
    }

    let tid = packet.type_id;
    match tid {
        0..=3 => {
            let values = packet.sub_packets.iter().map(do_calc);
            if tid == 0 {
                values.sum()
            } else if tid == 1 {
                values.product()
            } else if tid == 2 {
                values.min().expect("There's at least one sub-packet")
            } else {
                values.max().expect("There's at least one sub-packet")
            }
        }
        5..=7 => {
            let first = packet
                .sub_packets
                .first()
                .map(do_calc)
                .expect("There are always two sub-packets here");
            let second = packet
                .sub_packets
                .get(1)
                .map(do_calc)
                .expect("There are always two sub-packets here");

            let res = if packet.type_id == 5 {
                first > second
            } else if packet.type_id == 6 {
                first < second
            } else {
                first == second
            };
            u64::from(res)
        }
        _ => panic!("Unknown type id"),
    }
}

#[aoc(day16, part2)]
pub fn part2(inp: &str) -> u64 {
    let packet = parse_hex_packet(inp);
    do_calc(&packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal_value_packet() {
        let Some(gen) = generate("D2FE28") else {
            panic!("Could not parse test input")
        };

        let packet = parse_hex_packet(&gen);
        assert_eq!(packet.value, Some(2021));
    }

    #[test]
    fn test_parse_operator_packet_1() {
        let Some(gen) = generate("38006F45291200") else {
            panic!("Could not parse test input")
        };

        let packet = parse_hex_packet(&gen);
        assert_eq!(packet.sub_packets.len(), 2);
        assert_eq!(packet.sub_packets.first().and_then(|it| it.value), Some(10));
        assert_eq!(packet.sub_packets.get(1).and_then(|it| it.value), Some(20));
    }

    #[test]
    fn test_parse_operator_packet_2() {
        let Some(gen) = generate("EE00D40C823060") else {
            panic!("Could not parse test input")
        };

        let packet = parse_hex_packet(&gen);
        assert_eq!(packet.sub_packets.len(), 3);
        assert_eq!(packet.sub_packets.first().and_then(|it| it.value), Some(1));
        assert_eq!(packet.sub_packets.get(1).and_then(|it| it.value), Some(2));
        assert_eq!(packet.sub_packets.get(2).and_then(|it| it.value), Some(3));
    }

    #[test]
    fn test_sample_p1_1() {
        let Some(gen) = generate("8A004A801A8002F478") else {
            panic!("Could not parse test input")
        };

        let res = part1(&gen);
        assert_eq!(res, 16);
    }

    #[test]
    fn test_sample_p1_2() {
        let Some(gen) = generate("620080001611562C8802118E34") else {
            panic!("Could not parse test input")
        };

        let res = part1(&gen);
        assert_eq!(res, 12);
    }

    #[test]
    fn test_sample_p1_3() {
        let Some(gen) = generate("C0015000016115A2E0802F182340") else {
            panic!("Could not parse test input")
        };

        let res = part1(&gen);
        assert_eq!(res, 23);
    }

    #[test]
    fn test_sample_p2_1() {
        let test_data = [
            ("C200B40A82", 3u64),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (inp, expected) in test_data {
            let Some(gen) = generate(inp) else {
                panic!("Could not parse test input")
            };

            let res = part2(&gen);
            assert_eq!(res, expected);
        }
    }
}
