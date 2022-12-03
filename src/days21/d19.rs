use crate::{get_input, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Scanners = Vec<Vec<(i32, i32, i32)>>;

fn read(input: &str) -> Scanners {
    let b = |line: &str| {
        line.split(',')
            .map(str::parse::<i32>)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap()
    };
    let a = |scanner: &str| scanner.lines().skip(1).map(b).collect();
    input.split("\n\n").map(a).collect()
}

fn rotate_90((x, y, z): (i32, i32, i32)) -> [(i32, i32, i32); 4] {
    [
        (x, y, z),   // normal case
        (x, -z, y),  // rotate left 90 degrees
        (x, -y, -z), // rotate 180 degrees
        (x, z, -y),  // rotate right 90 degrees
    ]
}

const DIRS: [(i32, i32, i32); 8] = [
    (1, 1, 1),
    (1, -1, 1),
    (1, 1, -1),
    (1, -1, -1),
    (-1, 1, 1),
    (-1, -1, 1),
    (-1, 1, -1),
    (-1, -1, -1),
];

const ROT: [(i32, i32, i32); 4] = [(1, 1, 1), (1, -1, 1), (1, -1, -1), (1, 1, -1)];

// this doesn't work :(
fn find_beacons(scanners: Scanners) -> HashSet<(i32, i32, i32)> {
    let mut beacons = HashSet::from_iter(scanners[0].clone());

    'outer: for scanned in scanners.into_iter().skip(1) {
        let mut counts = HashMap::<(i32, i32, i32), i32>::new();

        for &(bx, by, bz) in &scanned {
            for &beacon in &beacons {
                for (i, (ax, ay, az)) in rotate_90(beacon).iter().enumerate() {
                    // let (ax, ay, az) = (beacon.0 * rx, beacon.1 * ry, beacon.2 * rz);
                    // println!("rotated: ({ax},{ay},{az})");
                    for (dx, dy, dz) in DIRS {
                        // formula: scannerX = bx + rotate(ax) * dx
                        let (sx, sy, sz) = (bx * dx + ax, by * dy + ay, bz * dz + az);
                        // if sx == 68 && sy == -1246 && sz == -43 {
                        //     println!("this iteration should have it\n");
                        // }
                        let count = counts.entry((sx, sy, sz)).or_default();
                        *count += 1;
                        if *count > 11 {
                            println!("\nfound scanner, it's at ({},{},{})", sx, sy, sz);
                            println!("dirs are at ({},{},{})", dx, dy, dz);
                            println!("rotated are at ({},{},{})", ax, ay, az);
                            // if found scanner: for all scanned, solve for a and insert to beacons
                            // formula: ax = (scannerx - bx) / direction
                            for &(cx, cy, cz) in &scanned {
                                let (cx, cy, cz) = rotate_90((cx, cy, cz))[i];
                                let transformed_a_coords =
                                    (sx - cx * dx, sy - cy * dy, sz - cz * dz);
                                beacons.insert(transformed_a_coords);
                            }
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    beacons
}

pub fn solve() -> Result<(i32, i32)> {
    let input = get_input(1)?;

    Ok((part1(&input)? as i32, part2(&input)? as i32))
}

fn part1(input: &str) -> Result<usize> {
    let scanners = read(input);
    let mut beacons: Vec<(i32, i32, i32)> = find_beacons(scanners).into_iter().collect();
    beacons.sort_unstable();
    println!("beacons: {:?}", beacons);

    Ok(beacons.len())
}

fn part2(input: &str) -> Result<i32> {
    Ok(0)
}

const INPUT: &str = include_str!("../../input/21/19_test.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(79, part1(INPUT).unwrap());
        assert_eq!(0, part2(INPUT).unwrap());
    }
}
