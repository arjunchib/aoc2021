use std::ops::Range;

fn main() {
    // Part 1
    assert_eq!(calc1("8A004A801A8002F478"), 16);
    assert_eq!(calc1("620080001611562C8802118E34"), 12);
    assert_eq!(calc1("C0015000016115A2E0802F182340"), 23);
    assert_eq!(calc1("A0016C880162017C3686B18A3D4780"), 31);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2("C200B40A82"), 3);
    assert_eq!(calc2("04005AC33890"), 54);
    assert_eq!(calc2("880086C3E88112"), 7);
    assert_eq!(calc2("CE00C43D881120"), 9);
    assert_eq!(calc2("D8005AC2A8F0"), 1);
    assert_eq!(calc2("F600BC2D8F"), 0);
    assert_eq!(calc2("9C005AC2F8F0"), 0);
    assert_eq!(calc2("9C0141080250320F1802104A08"), 1);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

struct BitVec {
    data: Vec<u8>,
}

impl BitVec {
    fn from(data: Vec<u8>) -> Self {
        BitVec { data }
    }

    fn get(&self, index: usize) -> usize {
        self.slice(index..index + 1)
    }

    fn slice(&self, range: Range<usize>) -> usize {
        let byte_range = range.start / 8..=(range.end - 1) / 8;
        let size = byte_range.end() - byte_range.start() + 1;
        if size > std::mem::size_of::<usize>() {
            panic!("Slice is out of bounds");
        }
        let prefix = std::mem::size_of::<usize>() - size;
        let mut bytes = vec![0; prefix];
        bytes.extend_from_slice(&self.data[byte_range]);
        let mut num = usize::from_be_bytes(bytes.try_into().unwrap());
        let lshf = prefix * 8 + (range.start % 8);
        num <<= lshf;
        let mut rshf = 8 - (range.end % 8);
        if rshf == 8 {
            rshf = 0
        }
        num >>= lshf + rshf;
        num
    }
}

struct Packet {
    end: usize,
    version: usize,
    type_id: usize,
    literal: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn from(data: &BitVec, start: usize) -> Self {
        let version = data.slice(start..start + 3);
        let type_id = data.slice(start + 3..start + 6);
        let mut sub_packets = vec![];
        let end;
        let mut literal = 0;
        if type_id == 4 {
            let mut i = start + 6;
            let mut vals = vec![];
            loop {
                vals.push(data.slice(i + 1..i + 5));
                if data.get(i) == 0 {
                    break;
                }
                i += 5;
            }
            literal = vals
                .iter()
                .rev()
                .enumerate()
                .map(|(i, x)| x << (i * 4))
                .sum();
            end = i + 5;
        } else {
            let length_type_id = data.get(start + 6);
            if length_type_id == 0 {
                let length = data.slice(start + 7..start + 22);
                let mut i = start + 22;
                while i < start + 22 + length {
                    let p = Packet::from(data, i);
                    i = p.end;
                    sub_packets.push(p);
                }
                end = i;
            } else {
                let length = data.slice(start + 7..start + 18);
                let mut i = start + 18;
                for _ in 0..length {
                    let p = Packet::from(data, i);
                    i = p.end;
                    sub_packets.push(p);
                }
                end = i;
            }
        }
        Packet {
            end,
            version,
            type_id,
            literal,
            sub_packets,
        }
    }

    fn total_versions(&self) -> usize {
        self.sub_packets
            .iter()
            .map(|x| x.total_versions())
            .sum::<usize>()
            + self.version
    }

    fn value(&self) -> usize {
        let packets = self.sub_packets.iter().map(|x| x.value());
        match self.type_id {
            0 => packets.sum(),
            1 => packets.product(),
            2 => packets.min().unwrap(),
            3 => packets.max().unwrap(),
            4 => self.literal,
            5 => {
                if self.sub_packets[0].value() > self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.sub_packets[0].value() < self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.sub_packets[0].value() == self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("invalid type"),
        }
    }
}

fn calc1(input: &str) -> usize {
    let mut data: Vec<u8> = vec![];
    for i in (0..input.len()).step_by(2) {
        data.push(u8::from_str_radix(&input[i..i + 2], 16).unwrap());
    }
    let b = BitVec::from(data);
    let p = Packet::from(&b, 0);
    p.total_versions()
}

fn calc2(input: &str) -> usize {
    let mut data: Vec<u8> = vec![];
    for i in (0..input.len()).step_by(2) {
        data.push(u8::from_str_radix(&input[i..i + 2], 16).unwrap());
    }
    let b = BitVec::from(data);
    let p = Packet::from(&b, 0);
    p.value()
}
