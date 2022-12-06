fn parse(input: &str) -> impl Iterator<Item = (u32, u32)> + '_ {
    fn lr((l, r): (&str, &str)) -> (u32, u32) {
        let res: Vec<_> = [["C", "B", "A"], ["X", "Y", "Z"]]
            .into_iter()
            .map(|a| a.into_iter().zip(1..=3))
            .zip([l, r])
            .filter_map(|(mut a, t)| a.find_map(|(v, n)| (v == t).then_some(n)))
            .collect();
        (res[0], res[1])
    }
    input
        .trim()
        .split('\n')
        .map(|ln| ln.split_once(' ').map(lr).unwrap())
}

fn part1(input: &str) -> u32 {
    parse(input).map(|(l, r)| r + 3 * ((l + r) % 3)).sum()
}

fn part2(input: &str) -> u32 {
    parse(input)
        .map(|(l, r)| (4 - l + r) % 3 + 1 + 3 * (r - 1))
        .sum()
}

crate::parts!(part1 part2);
