fn parse(input: &str) -> impl Iterator<Item = [std::ops::RangeInclusive<u32>; 2]> + '_ {
    input.lines().map(|l| {
        let mut iter = l.split(',').map(|r| {
            let (s, e) = r.split_once('-').unwrap();
            s.parse().unwrap()..=e.parse().unwrap()
        });
        [iter.next().unwrap(), iter.next().unwrap()]
    })
}

fn part1(input: &str) {
    let answer = parse(input)
        .filter(|p| {
            (0..=1).any(|i| p[i].start() <= p[1 - i].start() && p[i].end() >= p[1 - i].end())
        })
        .count();

    println!("{answer}");
}

fn part2(input: &str) {
    let answer = parse(input)
        .filter(|p| {
            (0..=1).any(|i| p[i].end() >= p[1 - i].start() && p[i].start() <= p[1 - i].end())
        })
        .count();

    println!("{answer}");
}

crate::parts!(part1 part2);
