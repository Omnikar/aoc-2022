fn parse(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    fn lr((l, r): (&str, &str)) -> (u32, u32) {
        (
            match l {
                "A" => 3,
                "B" => 2,
                "C" => 1,
                _ => panic!("malformed input"),
            },
            match r {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("malformed input"),
            },
        )
    }
    input
        .trim()
        .split('\n')
        .map(|ln| ln.split_once(' ').map(lr).unwrap())
}

fn part1(input: &str) {
    let answer: u32 = parse(input).map(|(l, r)| r + 3 * ((l + r) % 3)).sum();

    println!("{answer}");
}

fn part2(input: &str) {
    let answer: u32 = parse(input)
        .map(|(l, r)| (4 - l + r) % 3 + 1 + 3 * (r - 1))
        .sum();

    println!("{answer}");
}

crate::parts!(part1 part2);
