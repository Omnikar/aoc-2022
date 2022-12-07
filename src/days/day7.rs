use std::collections::HashMap;

type Dir = HashMap<String, Entry>;

enum Entry {
    File(usize),
    Dir(Dir),
}

impl Entry {
    fn size(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Dir(dir) => dir.values().map(Entry::size).sum(),
        }
    }
}

#[derive(Default)]
struct State {
    root: Dir,
    entered: Vec<*mut Dir>,
}

impl State {
    fn dir_mut(&mut self) -> &mut Dir {
        let ptr = self.entered.last().copied().unwrap_or(&mut self.root);
        unsafe { &mut *ptr }
    }

    fn enter_dir(&mut self, dir: &str) {
        let Entry::Dir(dir) = self.dir_mut().get_mut(dir).unwrap() else {
            panic!("malformed input");
        };
        let dir = dir as *mut _;
        self.entered.push(dir);
    }

    fn exit_dir(&mut self) -> bool {
        self.entered.pop().is_some()
    }
}

fn parse(input: &str) -> State {
    let mut state = State::default();

    let mut lines = input.lines().peekable();
    loop {
        let Some(l) = lines.next() else {
            break;
        };
        if let Some(cmd) = l.strip_prefix("$ ") {
            if let Some(dir) = cmd.strip_prefix("cd ") {
                match dir {
                    "/" => while state.exit_dir() {},
                    ".." => {
                        state.exit_dir();
                    }
                    dir => state.enter_dir(dir),
                }
            } else if cmd == "ls" {
                while lines.peek().map(|l| !l.starts_with("$ ")).unwrap_or(false) {
                    let l = lines.next().unwrap();
                    if let Some(dir) = l.strip_prefix("dir ") {
                        state
                            .dir_mut()
                            .insert(dir.to_owned(), Entry::Dir(Dir::default()));
                    } else {
                        let (size, file) = l.split_once(' ').unwrap();
                        state
                            .dir_mut()
                            .insert(file.to_owned(), Entry::File(size.parse().unwrap()));
                    }
                }
            }
        }
    }

    state
}

fn part1(input: &str) -> usize {
    let state = parse(input);

    fn solve_entry(e: &Entry) -> usize {
        match e {
            Entry::Dir(dir) => {
                dir.values().map(solve_entry).sum::<usize>()
                    + if let size @ 0..=100_000 = e.size() {
                        size
                    } else {
                        0
                    }
            }
            _ => 0,
        }
    }

    solve_entry(&Entry::Dir(state.root))
}

fn part2(input: &str) -> usize {
    let state = parse(input);

    let root_entry = Entry::Dir(state.root);

    let min_size = root_entry.size() - 40_000_000;

    fn solve_entry(e: &Entry, min_size: usize) -> usize {
        match e {
            Entry::File(_) => usize::MAX,
            Entry::Dir(dir) => dir
                .values()
                .map(|e| solve_entry(e, min_size))
                .min()
                .unwrap()
                .min({
                    let size = e.size();
                    if size >= min_size {
                        size
                    } else {
                        usize::MAX
                    }
                }),
        }
    }

    solve_entry(&root_entry, min_size)
}

crate::parts!(part1 part2);
