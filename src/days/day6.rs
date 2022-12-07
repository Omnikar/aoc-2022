fn solve(input: &str, marker_len: usize) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(marker_len)
        .enumerate()
        .find_map(|(i, w)| {
            (!w.iter()
                .any(|c1| w.iter().filter(|c2| *c1 == **c2).count() > 1))
            .then_some(i + marker_len)
        })
        .unwrap()
}

fn part1(input: &str) -> usize {
    solve(input, 4)
}

fn part2(input: &str) -> usize {
    solve(input, 14)
}

crate::parts!(part1 part2);
