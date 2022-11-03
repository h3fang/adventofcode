#[derive(Debug, PartialEq)]
struct Header {
    version: u8,
    type_id: u8,
}

#[derive(Debug, PartialEq)]
enum Body {
    Literal(i64),
    Operator(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    header: Header,
    body: Body,
}

impl Packet {
    fn versions(&self) -> i64 {
        match &self.body {
            Body::Literal(_) => self.header.version as i64,
            Body::Operator(pkts) => {
                self.header.version as i64 + pkts.iter().map(|p| p.versions()).sum::<i64>()
            }
        }
    }

    fn value(&self) -> i64 {
        match &self.body {
            Body::Literal(v) => *v,
            Body::Operator(sub_packets) => match self.header.type_id {
                0 => sub_packets.iter().map(|pkt| pkt.value()).sum(),
                1 => sub_packets.iter().map(|pkt| pkt.value()).product(),
                2 => sub_packets
                    .iter()
                    .map(|pkt| pkt.value())
                    .min()
                    .expect("empty sub-packets for Minimum operator"),
                3 => sub_packets
                    .iter()
                    .map(|pkt| pkt.value())
                    .max()
                    .expect("empty sub-packets for Maximum operator"),
                5 => (sub_packets[0].value() > sub_packets[1].value()).into(),
                6 => (sub_packets[0].value() < sub_packets[1].value()).into(),
                7 => (sub_packets[0].value() == sub_packets[1].value()).into(),
                _ => unreachable!(),
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
                result = (result << 1) + u16::from(bit);
            }
            self.idx += n;
            Ok(result)
        } else {
            Err(BITSReadError::NotEnoughBits)
        }
    }

    fn read_packet(&mut self) -> Result<Packet, BITSReadError> {
        let version = self.read(3)? as u8;
        let type_id = self.read(3)? as u8;
        let header = Header { version, type_id };
        match header.type_id {
            4 => {
                let mut value = 0i64;
                loop {
                    let prefix = self.read(1)?;
                    let group = self.read(4)? as i64;
                    value = (value << 4) + group;
                    if prefix == 0 {
                        break;
                    }
                }
                Ok(Packet {
                    header,
                    body: Body::Literal(value),
                })
            }
            _ => {
                let len_type = self.read(1)?;
                let sub_packets = match len_type {
                    0 => {
                        let mut len = self.read(15)? as usize;
                        let mut sub_pkts = vec![];
                        while len > 0 {
                            let curr = self.idx;
                            sub_pkts.push(self.read_packet()?);
                            len -= self.idx - curr;
                        }
                        sub_pkts
                    }
                    _ => {
                        let len = self.read(11)? as usize;
                        let sub_pkts: Result<Vec<_>, _> =
                            (0..len).map(|_| self.read_packet()).collect();
                        sub_pkts?
                    }
                };
                Ok(Packet {
                    header,
                    body: Body::Operator(sub_packets),
                })
            }
        }
    }
}

fn solve(data: &[u8]) -> (i64, i64) {
    let mut bits = BITSTransmission::new(data);
    if let Ok(pkt) = bits.read_packet() {
        (pkt.versions(), pkt.value())
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
            Ok(Packet {
                header: Header {
                    version: 6,
                    type_id: 4
                },
                body: Body::Literal(2021),
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
            Ok(Packet {
                header: Header {
                    version: 1,
                    type_id: 6,
                },
                body: Body::Operator(vec![
                    Packet {
                        header: Header {
                            version: 6,
                            type_id: 4
                        },
                        body: Body::Literal(10),
                    },
                    Packet {
                        header: Header {
                            version: 2,
                            type_id: 4
                        },
                        body: Body::Literal(20),
                    }
                ]),
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
