enum Instr {
    Addx(i32),
    Noop,
}

fn parse(input: &str) -> impl Iterator<Item = Instr> + '_ {
    input.trim().lines().map(|l| {
        if let Some(s) = l.strip_prefix("addx ") {
            Instr::Addx(s.parse().unwrap())
        } else {
            Instr::Noop
        }
    })
}

fn part1(input: &str) -> i32 {
    let mut x = 1;
    let mut strengths = Vec::new();
    let mut cycle = |x| strengths.push(x * (strengths.len() + 1) as i32);
    parse(input).for_each(|instr| match instr {
        Instr::Addx(n) => {
            cycle(x);
            cycle(x);
            x += n;
        }
        Instr::Noop => cycle(x),
    });
    strengths
        .drain(..220)
        .enumerate()
        .filter_map(|(i, n)| (i % 40 == 19).then_some(n))
        .sum()
}

crate::parts!(part1);
