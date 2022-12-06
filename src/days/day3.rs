fn priority(c: char) -> u32 {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(c)
        .unwrap() as u32
        + 1
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(l, r)| {
            l.chars()
                .flat_map(|lc| r.chars().map(move |rc| (lc, rc)))
                .find_map(|(a, b)| (a == b).then_some(a))
                .unwrap()
        })
        .map(priority)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|g| {
            g.iter()
                .map(|s| s.chars().collect::<Vec<_>>())
                .reduce(|mut possible, next| {
                    possible.retain(|c| next.contains(c));
                    possible
                })
                .unwrap()[0]
        })
        .map(priority)
        .sum()
}

crate::parts!(part1 part2);
