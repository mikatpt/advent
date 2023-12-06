use crate::{
    eyre, {get_input, Result},
};

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn handle_packet(packet: String, part1: bool) -> Result<(usize, u64)> {
    let t = u64::from_str_radix(&packet[3..6], 2)?;
    match t {
        4 => handle_literal(&packet, part1),
        _ => handle_operation(packet, part1),
    }
}

fn handle_literal(packet: &str, part1: bool) -> Result<(usize, u64)> {
    let version = u64::from_str_radix(&packet[..3], 2)?;

    let mut num = String::new();
    let (mut bits, mut len) = (&packet[6..], 6);

    loop {
        len += 5;
        num.push_str(&bits[1..5]);
        if bits.starts_with('0') {
            break;
        }
        bits = &bits[5..];
    }
    let res = if part1 {
        version
    } else {
        u64::from_str_radix(&num, 2)?
    };

    Ok((len, res))
}

fn handle_operation(mut packet: String, part1: bool) -> Result<(usize, u64)> {
    let version = u64::from_str_radix(&packet[..3], 2)?;
    let type_id = u64::from_str_radix(&packet[3..6], 2)?;

    let mut results = vec![];
    let mut size = 18;

    match packet
        .chars()
        .nth(6)
        .ok_or_else(|| eyre!("if bit 6 does not exist this is not a valid packet."))?
    {
        '0' => {
            size = usize::from_str_radix(&packet[7..22], 2)?;
            packet = packet[22..22 + size].to_string();
            loop {
                let (c, r) = handle_packet(packet.clone(), part1)?;
                results.push(r);
                packet = packet[c..].to_string();

                if packet.len() < 11 {
                    size += 22 - packet.len();
                    break;
                }
            }
        }
        _ => {
            let sub_packets = u64::from_str_radix(&packet[7..18], 2)? as usize;
            packet = packet[18..].to_string();
            for i in 0..sub_packets {
                let (count, result) = handle_packet(packet.clone(), part1)?;
                results.push(result);
                size += count;
                packet = packet[count..].to_string();
            }
        }
    };

    let ops = apply_operation(type_id, results);
    let res = if part1 { version + ops } else { ops };
    Ok((size, res))
}

fn apply_operation(type_id: u64, results: Vec<u64>) -> u64 {
    match type_id {
        0 => results.into_iter().sum(),
        1 => results.into_iter().product(),
        2 => results.into_iter().min().unwrap(),
        3 => results.into_iter().max().unwrap(),
        5 => {
            if results[0] > results[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if results[0] < results[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if results[0] == results[1] {
                1
            } else {
                0
            }
        }
        _ => results.into_iter().sum(),
    }
}

fn read_hex_into_binary(hex: &str) -> String {
    hex.chars()
        .map(|c| c.to_digit(16).expect("This should always be a hex digit."))
        .fold(String::new(), |prev, digit| format!("{prev}{:04b}", digit))
}

pub fn part1(input: &str) -> Result<u64> {
    let packet = read_hex_into_binary(input);
    let (_, res) = handle_packet(packet, true)?;
    Ok(res)
}

fn part2(input: &str) -> Result<u64> {
    let packet = read_hex_into_binary(input);
    let (_, res) = handle_packet(packet, false)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(16, part1("8A004A801A8002F478").unwrap());
        assert_eq!(3, part2("C200B40A82").unwrap());
    }

    #[test]
    fn test1_2() {
        assert_eq!(12, part1("620080001611562C8802118E34").unwrap());
    }

    #[test]
    fn test1_3() {
        assert_eq!(23, part1("C0015000016115A2E0802F182340").unwrap());
    }

    #[test]
    fn test1_4() {
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780").unwrap());
    }
}
