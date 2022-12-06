fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> + '_ {
    input.split("\n\n").map(|inv| {
        inv.split_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap)
    })
}

fn part1(input: &str) -> u32 {
    parse(input).map(Iterator::sum).max().unwrap_or(0)
}

fn part2(input: &str) -> u32 {
    let mut invs: Vec<u32> = parse(input).map(Iterator::sum).collect();
    invs.sort();
    invs.drain((invs.len() - 3)..).sum()
}

crate::parts!(part1 part2);
