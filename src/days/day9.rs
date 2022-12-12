use std::collections::HashSet;

fn parse(input: &str) -> impl Iterator<Item = [i32; 2]> + '_ {
    input.lines().map(|l| {
        let (dir, dist) = l.split_once(' ').unwrap();
        let dist: i32 = dist.parse().unwrap();
        match dir {
            "R" => [1, 0],
            "L" => [-1, 0],
            "U" => [0, 1],
            "D" => [0, -1],
            _ => panic!("malformed input"),
        }
        .map(|d| d * dist)
    })
}

fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let [mut head, mut tail] = [[0, 0]; 2];
    visited.insert(tail);

    parse(input).for_each(|delta| {
        head.iter_mut().zip(delta).for_each(|(h, d)| *h += d);
        let mut diff = head;
        diff.iter_mut().zip(tail).for_each(|(d, t)| *d -= t);
        let Some(n) = [0,1].into_iter().find(|&n| diff[n].abs() >= 2) else { return };
        let dist = diff[n] - diff[n].signum();
        tail[n] += dist;
        if diff[1 - n].abs() >= 1 {
            tail[1 - n] += diff[1 - n] / diff[1 - n].abs();
        }
        (0..dist.abs()).for_each(|d| {
            let mut tmp_tail = tail;
            tmp_tail[n] -= d * dist.signum();
            visited.insert(tmp_tail);
        });
    });

    visited.len()
}

crate::parts!(part1);
