use std::iter::repeat;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
        .map(Iterator::collect)
        .collect()
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    (0..grid.len())
        .zip(grid.iter().map(Vec::len))
        .flat_map(|(x, l)| repeat(x).zip(0..l))
        .map(|(x, y)| (x, y, grid[x][y]))
        .filter(|&(x, y, h)| {
            [
                Box::new((0..x).zip(repeat(y))) as Box<dyn Iterator<Item = _>>,
                Box::new((x + 1..).zip(repeat(y))),
                Box::new(repeat(x).zip(0..y)),
                Box::new(repeat(x).zip(y + 1..)),
            ]
            .into_iter()
            .any(|iter| {
                iter.map(|(x, y)| grid.get(x).and_then(|col| col.get(y)))
                    .take_while(Option::is_some)
                    .flatten()
                    .all(|ch| *ch < h)
            })
        })
        .count()
}

fn part2(input: &str) -> usize {
    let grid = parse(input);
    (0..grid.len())
        .zip(grid.iter().map(Vec::len))
        .flat_map(|(x, l)| repeat(x).zip(0..l))
        .map(|(x, y)| {
            let h = grid[x][y];
            [
                Box::new((0..x).rev().zip(repeat(y))) as Box<dyn Iterator<Item = _>>,
                Box::new((x + 1..).zip(repeat(y))),
                Box::new(repeat(x).zip((0..y).rev())),
                Box::new(repeat(x).zip(y + 1..)),
            ]
            .into_iter()
            .map(|r| {
                let mut end = false;
                r.map(|(x, y)| grid.get(x).and_then(|col| col.get(y)))
                    .take_while(Option::is_some)
                    .flatten()
                    .take_while(|ch| {
                        let cur_end = end;
                        end = **ch >= h;
                        !cur_end
                    })
                    .count()
            })
            .product()
        })
        .max()
        .unwrap()
}

crate::parts!(part1 part2);
