fn parse(input: &str) -> (Vec<Vec<char>>, impl Iterator<Item = [usize; 3]> + '_) {
    let (stacks_s, instrs_s) = input.split_once("\n\n").unwrap();

    let stacks = stacks_s
        .lines()
        .rev()
        .skip(1)
        .map(|l| {
            l.chars()
                .enumerate()
                .filter_map(|(i, c)| (i % 4 == 1).then_some(c))
                .enumerate()
                .filter(|(_, c)| *c != ' ')
        })
        .fold(Vec::new(), |mut stks, row| {
            row.for_each(|(i, o)| {
                while stks.len() <= i {
                    stks.push(Vec::new())
                }
                stks[i].push(o)
            });
            stks
        });

    let instrs = instrs_s.lines().map(|l| {
        let mut iter = l
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .enumerate()
            .map(|(i, n)| n - (i + 1) / 2);
        let mut arr = [0; 3];
        arr.fill_with(|| iter.next().unwrap());
        arr
    });

    (stacks, instrs)
}

fn part1(input: &str) -> String {
    let (mut stacks, instrs) = parse(input);

    instrs
        .flat_map(|[q, f, t]| std::iter::repeat([f, t]).take(q))
        .for_each(|[f, t]| {
            let ct = stacks[f].pop().unwrap();
            stacks[t].push(ct);
        });

    stacks.into_iter().map(|stk| *stk.last().unwrap()).collect()
}

fn part2(input: &str) -> String {
    let (mut stacks, instrs) = parse(input);

    instrs.for_each(|[q, f, t]| {
        let i = stacks[f].len() - q;
        let mut cts = stacks[f].drain(i..).collect();
        stacks[t].append(&mut cts);
    });

    stacks.into_iter().map(|stk| *stk.last().unwrap()).collect()
}

crate::parts!(part1 part2);
