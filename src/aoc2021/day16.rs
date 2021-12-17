#[derive(Debug, PartialEq)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u16> for PacketType {
    fn from(n: u16) -> Self {
        match n & 0b111 {
            4 => PacketType::Literal,
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Header {
    version: u8,
    type_id: PacketType,
}

#[derive(Debug, PartialEq)]
enum LengthType {
    TotalBits,
    TotalPackets,
}

impl From<u16> for LengthType {
    fn from(n: u16) -> Self {
        match n & 0b1 {
            0 => LengthType::TotalBits,
            1 => LengthType::TotalPackets,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal {
        header: Header,
        value: i64,
    },
    Operator {
        header: Header,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn value(&self) -> i64 {
        match self {
            Packet::Literal { header: _, value } => *value,
            Packet::Operator {
                header,
                sub_packets,
            } => match header.type_id {
                PacketType::Literal => unreachable!(),
                PacketType::Sum => sub_packets.iter().map(|pkt| pkt.value()).sum(),
                PacketType::Product => sub_packets.iter().map(|pkt| pkt.value()).product(),
                PacketType::Minimum => sub_packets
                    .iter()
                    .map(|pkt| pkt.value())
                    .min()
                    .expect("empty sub-packets for Minimum operator"),
                PacketType::Maximum => sub_packets
                    .iter()
                    .map(|pkt| pkt.value())
                    .max()
                    .expect("empty sub-packets for Maximum operator"),
                PacketType::GreaterThan => {
                    if sub_packets[0].value() > sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                PacketType::LessThan => {
                    if sub_packets[0].value() < sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                PacketType::EqualTo => {
                    if sub_packets[0].value() == sub_packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

struct BITSTransmission {
    data: Vec<bool>,
    idx: usize,
}

#[derive(Debug, PartialEq)]
enum BITSReadError {
    ExceededMaximumReadSize,
    NotEnoughBits,
}

impl BITSTransmission {
    fn new(bin: &[u8]) -> Self {
        let mut data = Vec::with_capacity(bin.len() * 8);
        for b in bin {
            let b = (*b as char).to_digit(16).expect("invalid hexadecimal byte") as u8;
            let mut mask = 1u8 << 3;
            while mask > 0 {
                data.push(mask & b > 0);
                mask >>= 1;
            }
        }
        Self { data, idx: 0 }
    }

    fn read(&mut self, n: usize) -> Result<u16, BITSReadError> {
        if n > 15 {
            Err(BITSReadError::ExceededMaximumReadSize)
        } else if n + self.idx <= self.data.len() {
            let mut result = 0;
            for &bit in &self.data[self.idx..self.idx + n] {
                result = (result << 1) + if bit { 1 } else { 0 };
            }
            self.idx += n;
            Ok(result)
        } else {
            Err(BITSReadError::NotEnoughBits)
        }
    }

    fn read_packet(&mut self) -> Result<Packet, BITSReadError> {
        let version = self.read(3)? as u8;
        let type_id = self.read(3)?.into();
        let header = Header { version, type_id };
        match header.type_id {
            PacketType::Literal => {
                let mut value = 0i64;
                loop {
                    let prefix = self.read(1)?;
                    let group = self.read(4)? as i64;
                    value = (value << 4) + group;
                    if prefix == 0 {
                        break;
                    } else if prefix != 1 {
                        panic!("invalid literal group prefix");
                    }
                }
                Ok(Packet::Literal { header, value })
            }
            _ => {
                let len_type: LengthType = self.read(1)?.into();
                let sub_packets = match len_type {
                    LengthType::TotalBits => {
                        let mut len = self.read(15)? as usize;
                        let mut sub_pkts = vec![];
                        while len > 0 {
                            let curr = self.idx;
                            sub_pkts.push(self.read_packet()?);
                            len -= self.idx - curr;
                        }
                        sub_pkts
                    }
                    LengthType::TotalPackets => {
                        let len = self.read(11)? as usize;
                        let sub_pkts: Result<Vec<_>, _> =
                            (0..len).map(|_| self.read_packet()).collect();
                        sub_pkts?
                    }
                };
                Ok(Packet::Operator {
                    header,
                    sub_packets,
                })
            }
        }
    }
}

fn solve(data: &[u8]) -> (usize, i64) {
    fn accumulate_versions(pkt: &Packet) -> usize {
        match pkt {
            Packet::Literal { header, value: _ } => header.version as usize,
            Packet::Operator {
                header,
                sub_packets,
            } => {
                let mut result = header.version as usize;
                for sub in sub_packets {
                    result += accumulate_versions(sub);
                }
                result
            }
        }
    }
    let mut bits = BITSTransmission::new(data);
    if let Ok(pkt) = bits.read_packet() {
        (accumulate_versions(&pkt), pkt.value())
    } else {
        unreachable!()
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day16").unwrap();
    let (p1, p2) = solve(data.trim().as_bytes());
    println!("day16 part1: {}", p1);
    println!("day16 part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "D2FE28";
        let mut bits = BITSTransmission::new(data.as_bytes());
        let pkt = bits.read_packet();
        assert_eq!(
            Ok(Packet::Literal {
                header: Header {
                    version: 6,
                    type_id: PacketType::Literal
                },
                value: 2021,
            }),
            pkt
        );
    }

    #[test]
    fn case2() {
        let data = "38006F45291200";
        let mut bits = BITSTransmission::new(data.as_bytes());
        let pkt = bits.read_packet();
        assert_eq!(
            Ok(Packet::Operator {
                header: Header {
                    version: 1,
                    type_id: PacketType::LessThan
                },
                sub_packets: vec![
                    Packet::Literal {
                        header: Header {
                            version: 6,
                            type_id: PacketType::Literal
                        },
                        value: 10,
                    },
                    Packet::Literal {
                        header: Header {
                            version: 2,
                            type_id: PacketType::Literal
                        },
                        value: 20,
                    }
                ],
            }),
            pkt
        );
    }

    #[test]
    fn case3() {
        let data = "8A004A801A8002F478";
        assert_eq!(16, solve(data.as_bytes()).0);
    }

    #[test]
    fn case4() {
        let data = "620080001611562C8802118E34";
        assert_eq!(12, solve(data.as_bytes()).0);
    }

    #[test]
    fn case5() {
        let data = "C0015000016115A2E0802F182340";
        assert_eq!(23, solve(data.as_bytes()).0);
    }

    #[test]
    fn case6() {
        let data = "A0016C880162017C3686B18A3D4780";
        assert_eq!(31, solve(data.as_bytes()).0);
    }

    #[test]
    fn case7() {
        let data = "C200B40A82";
        assert_eq!(3, solve(data.as_bytes()).1);
    }

    #[test]
    fn case8() {
        let data = "04005AC33890";
        assert_eq!(54, solve(data.as_bytes()).1);
    }

    #[test]
    fn case9() {
        let data = "880086C3E88112";
        assert_eq!(7, solve(data.as_bytes()).1);
    }

    #[test]
    fn case10() {
        let data = "CE00C43D881120";
        assert_eq!(9, solve(data.as_bytes()).1);
    }

    #[test]
    fn case11() {
        let data = "D8005AC2A8F0";
        assert_eq!(1, solve(data.as_bytes()).1);
    }

    #[test]
    fn case12() {
        let data = "F600BC2D8F";
        assert_eq!(0, solve(data.as_bytes()).1);
    }

    #[test]
    fn case13() {
        let data = "9C005AC2F8F0";
        assert_eq!(0, solve(data.as_bytes()).1);
    }

    #[test]
    fn case14() {
        let data = "9C0141080250320F1802104A08";
        assert_eq!(1, solve(data.as_bytes()).1);
    }
}
