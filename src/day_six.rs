pub fn part_one(input: &str) -> usize {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    time.iter()
        .zip(distance.iter())
        .map(|(t, d)| ways_to_win_single(*t, *d))
        .product()
}

pub fn part_two(input: &str) -> usize {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<usize>()
        .unwrap();

    ways_to_win_single(time, distance)
}

/// Let `a` be the duration of a race. Let `k` be the max distance.
/// returns an integer of the number of ways the race can be won.
fn ways_to_win_single(a: usize, k: usize) -> usize {
    let discriminant = a.pow(2) - 4 * k;
    let sqrt_discriminant = (discriminant as f64).sqrt();
    let x1 = (a as f64 + sqrt_discriminant) / 2.0;
    let x2 = (a as f64 - sqrt_discriminant) / 2.0;

    let min = x1.min(x2);
    let max = x1.max(x2);

    max.ceil() as usize - min.floor() as usize - 1
}
