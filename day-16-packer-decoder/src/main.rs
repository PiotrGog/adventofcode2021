use std::{collections::LinkedList, fs};
use to_binary::{self, BinaryString};

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Packet {
    pub fn create(data: &str) -> Option<(Self, &str)> {
        if data.len() < 6 {
            return None;
        }

        let packet_version = &data[0..3];
        let type_id = &data[3..6];
        let packet_data_and_rest = &data[6..];
        let packet_version = u8::from_str_radix(packet_version, 2).unwrap();
        let type_id = u8::from_str_radix(type_id, 2).unwrap();

        let packet_and_rest = match type_id {
            4u8 => {
                Self::create_literal_packet_and_return_rest(packet_version, packet_data_and_rest)
            }
            _ => Self::create_operator_packet_and_return_rest(packet_version, packet_data_and_rest),
        };

        Some(packet_and_rest)
    }

    fn create_literal_packet_and_return_rest(
        packet_version: u8,
        packet_data_and_rest: &str,
    ) -> (Packet, &str) {
        let mut num_of_values = 0;
        let mut value = vec![];

        loop {
            let slice_start = 5 * num_of_values;
            let string_value = &packet_data_and_rest[slice_start..(slice_start + 5)];
            let is_last_part = string_value.chars().nth(0).unwrap() == '0';
            value.push(u8::from_str_radix(&string_value[1..], 2).unwrap());
            num_of_values += 1;

            if is_last_part {
                break;
            }
        }

        (
            Packet::Literal(LiteralPacket::new(packet_version, value)),
            &packet_data_and_rest[(num_of_values * 5)..],
        )
    }

    fn create_operator_packet_and_return_rest(
        packet_version: u8,
        packet_data_and_rest: &str,
    ) -> (Packet, &str) {
        let length_type_id = packet_data_and_rest.chars().nth(0).unwrap();
        let subpackets_in_bit_length = length_type_id == '0';
        let (subpackets, rest) = if subpackets_in_bit_length {
            Self::process_operator_packet_with_total_length_in_bits(&packet_data_and_rest[1..])
        } else {
            Self::process_operator_packet_with_number_of_subpackets_contained(
                &packet_data_and_rest[1..],
            )
        };

        (
            Packet::Operator(OperatorPacket::new(packet_version, subpackets)),
            rest,
        )
    }

    fn process_operator_packet_with_total_length_in_bits(
        packet_data_and_rest: &str,
    ) -> (Vec<Packet>, &str) {
        let length_value_number_of_bits = 15;
        let length_of_bits_for_subpackets =
            usize::from_str_radix(&packet_data_and_rest[..length_value_number_of_bits], 2).unwrap();
        let subpackets_and_rest = &packet_data_and_rest[length_value_number_of_bits..];

        let mut subpackets_data = &subpackets_and_rest[..length_of_bits_for_subpackets];
        let mut subpackets = vec![];
        while let Some((packet, rest)) = Self::create(subpackets_data) {
            subpackets_data = rest;
            subpackets.push(packet);
        }

        (
            subpackets,
            &subpackets_and_rest[length_of_bits_for_subpackets..],
        )
    }

    fn process_operator_packet_with_number_of_subpackets_contained(
        packet_data_and_rest: &str,
    ) -> (Vec<Packet>, &str) {
        let number_of_subpackets_number_of_bits = 11;
        let number_of_subpackets = usize::from_str_radix(
            &packet_data_and_rest[..number_of_subpackets_number_of_bits],
            2,
        )
        .unwrap();
        let mut subpackets_and_rest = &packet_data_and_rest[number_of_subpackets_number_of_bits..];
        let mut subpackets = vec![];
        for _ in 0..number_of_subpackets {
            let (packet, rest) = Self::create(subpackets_and_rest).unwrap();
            subpackets_and_rest = rest;
            subpackets.push(packet);
        }

        (subpackets, subpackets_and_rest)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LiteralPacket {
    version: u8,
    value: Vec<u8>,
}

impl LiteralPacket {
    pub fn new(version: u8, value: Vec<u8>) -> Self {
        Self { version, value }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct OperatorPacket {
    version: u8,
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    pub fn new(version: u8, subpackets: Vec<Packet>) -> Self {
        Self {
            version,
            subpackets,
        }
    }
}

struct BITSTransmision {
    packet: Option<Packet>,
}

impl BITSTransmision {
    pub fn from_hex_string(hex: &str) -> Self {
        let binary_bits_transmision =
            BinaryString::from_hex(hex).expect("Cannot parse given hex string");
        Self::from_bin_string(&binary_bits_transmision.0)
    }

    pub fn from_bin_string(bin: &str) -> Self {
        Self {
            packet: Some(Packet::create(bin).unwrap().0),
        }
    }

    pub fn sum_up_versions(&self) -> Option<usize> {
        if self.packet == None {
            return None;
        }

        let mut stack = LinkedList::new();
        stack.push_front(self.packet.as_ref().unwrap());
        let mut version_sum = 0;
        while let Some(packet) = stack.pop_front() {
            match packet {
                Packet::Literal(LiteralPacket { version, .. }) => version_sum += *version as usize,
                Packet::Operator(OperatorPacket {
                    version,
                    subpackets,
                }) => {
                    version_sum += *version as usize;
                    for subpacket in subpackets {
                        stack.push_front(subpacket);
                    }
                }
            }
        }

        Some(version_sum)
    }
}

fn load_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Should have been able to read the file")
}

fn part_1_result(file_name: &str) {
    let bits_transmission_data = load_file(file_name);
    let bits_transmission = BITSTransmision::from_hex_string(&bits_transmission_data);
    println!(
        "Part 1. Result: {}",
        bits_transmission.sum_up_versions().unwrap()
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_string_crate_convert_hex_string_to_binary_string() {
        let binary_representation = to_binary::BinaryString::from_hex("D2FE28").unwrap().0;
        assert_eq!(binary_representation, "110100101111111000101000");
    }

    #[test]
    fn packet_create_return_literal() {
        let (result_packet, rest) = Packet::create("110100101111111000101000").unwrap();
        assert_eq!(
            result_packet,
            Packet::Literal(LiteralPacket {
                version: 6,
                value: vec![0b0111, 0b1110, 0b0101]
            })
        );
        assert_eq!(rest, "000".to_string());
    }

    #[test]
    fn packet_create_return_operator_with_length_type_id_0() {
        let (result_packet, rest) =
            Packet::create("00111000000000000110111101000101001010010001001000000000").unwrap();
        assert_eq!(
            result_packet,
            Packet::Operator(OperatorPacket {
                version: 1,
                subpackets: vec![
                    Packet::Literal(LiteralPacket {
                        version: 6,
                        value: vec![0b1010]
                    }),
                    Packet::Literal(LiteralPacket {
                        version: 2,
                        value: vec![0b0001, 0b0100]
                    })
                ]
            })
        );
        assert_eq!(rest, "0000000");
    }

    #[test]
    fn packet_create_return_operator_with_length_type_id_1() {
        let (result_packet, rest) =
            Packet::create("11101110000000001101010000001100100000100011000001100000").unwrap();
        assert_eq!(
            result_packet,
            Packet::Operator(OperatorPacket {
                version: 7,
                subpackets: vec![
                    Packet::Literal(LiteralPacket {
                        version: 2,
                        value: vec![0b0001]
                    }),
                    Packet::Literal(LiteralPacket {
                        version: 4,
                        value: vec![0b0010]
                    }),
                    Packet::Literal(LiteralPacket {
                        version: 1,
                        value: vec![0b0011]
                    })
                ]
            })
        );
        assert_eq!(rest, "00000");
    }

    #[test]
    fn sum_up_versions_tc1() {
        let bits_transmission_data = "8A004A801A8002F478";
        let bits_transmission = BITSTransmision::from_hex_string(bits_transmission_data);
        assert_eq!(bits_transmission.sum_up_versions().unwrap(), 16);
    }

    #[test]
    fn sum_up_versions_tc2() {
        let bits_transmission_data = "620080001611562C8802118E34";
        let bits_transmission = BITSTransmision::from_hex_string(bits_transmission_data);
        assert_eq!(bits_transmission.sum_up_versions().unwrap(), 12);
    }

    #[test]
    fn sum_up_versions_tc3() {
        let bits_transmission_data = "C0015000016115A2E0802F182340";
        let bits_transmission = BITSTransmision::from_hex_string(bits_transmission_data);
        assert_eq!(bits_transmission.sum_up_versions().unwrap(), 23);
    }

    #[test]
    fn sum_up_versions_tc4() {
        let bits_transmission_data = "A0016C880162017C3686B18A3D4780";
        let bits_transmission = BITSTransmision::from_hex_string(bits_transmission_data);
        assert_eq!(bits_transmission.sum_up_versions().unwrap(), 31);
    }
}
