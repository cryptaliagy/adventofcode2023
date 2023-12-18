fn hash(sequence: &str) -> u8 {
    let mut current = 0u8;

    for c in sequence.bytes() {
        current = current.wrapping_add(c);
        current = current.wrapping_mul(17);
    }

    current
}

enum Initialization<'a> {
    Removal(&'a str),
    Lens(&'a str, usize, bool),
}

pub fn part_one(input: &str) -> i64 {
    input
        .split(',')
        .map(|x| x.trim())
        .map(hash)
        .fold(0i64, |acc, x| acc + (x as i64))
}

pub fn part_two(input: &str) -> i64 {
    let sequence = input.split(',').map(|x| {
        let x = x.trim();

        if x.ends_with('-') {
            Initialization::Removal(&x[..x.len() - 1])
        } else {
            let (label, length) = x.split_once('=').unwrap();
            let length = length.parse::<usize>().unwrap();

            Initialization::Lens(label, length, true)
        }
    });

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part_one(input), 1320);
    }
}
