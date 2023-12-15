fn hash(sequence: &str) -> u8 {
    let mut current = 0u8;

    for c in sequence.bytes() {
        current = current.wrapping_add(c);
        current = current.wrapping_mul(17);
    }

    current
}

pub fn part_one(input: &str) -> i64 {
    input
        .split(',')
        .map(|x| x.trim())
        .map(hash)
        .fold(0i64, |acc, x| acc + (x as i64))
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
