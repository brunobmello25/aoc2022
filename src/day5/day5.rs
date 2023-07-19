use crate::Day;

pub struct Day5 {}

impl Day for Day5 {
    fn run(&self, part: &'static str) {
        match part {
            "part1" => self.run_part_1(),
            "part2" => self.run_part_2(),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
impl Day5 {
    pub fn new() -> Self {
        Day5 {}
    }

    fn run_part_1(&self) {
        let input = self.read_input("inputs/day5.txt");
        let splitted = input.split("\n\n").collect::<Vec<&str>>();
        let state = splitted[0].to_string();
        let movements = splitted[1].to_string();

        let mut stacks = self.parse_initial_state(state);
        let movements = self.parse_moves(movements);

        self.run_moves(&mut stacks, movements, false);

        for stack in stacks {
            println!("{}", stack.top().unwrap());
        }
    }

    fn run_part_2(&self) {
        let input = self.read_input("inputs/day5.txt");
        let splitted = input.split("\n\n").collect::<Vec<&str>>();
        let state = splitted[0].to_string();
        let movements = splitted[1].to_string();

        let mut stacks = self.parse_initial_state(state);
        let movements = self.parse_moves(movements);

        self.run_moves(&mut stacks, movements, true);

        for stack in stacks {
            println!("{}", stack.top().unwrap());
        }
    }

    fn read_input(&self, filename: &str) -> String {
        std::fs::read_to_string(filename).expect("failed to read input content")
    }

    fn parse_initial_state(&self, mut state: String) -> Vec<Stack> {
        if state.ends_with("\n") {
            state.pop();
        }

        let matrix: Vec<Vec<char>> = state
            .split("\n")
            .map(|line| line.chars().collect())
            .collect();

        let mut stacks: Vec<Stack> = vec![];

        for (col, ch) in matrix[matrix.len() - 1].iter().enumerate() {
            if !ch.is_numeric() {
                continue;
            }

            let mut stack = Stack::new(ch.to_string().parse().unwrap());

            for line in (0..(matrix.len() - 1)).rev() {
                let crate_symbol = matrix[line][col];

                if crate_symbol == ' ' {
                    break;
                }

                stack.push(crate_symbol);
            }

            stacks.push(stack);
        }

        return stacks;
    }

    fn parse_moves(&self, moves: String) -> Vec<Movement> {
        let moves = moves.trim().split("\n");

        let mut movements: Vec<Movement> = vec![];
        for movement in moves {
            movements.push(Movement::new(movement.to_string()));
        }

        return movements;
    }

    fn run_moves(&self, stacks: &mut Vec<Stack>, movements: Vec<Movement>, keep_order: bool) {
        for movement in movements.iter() {
            let from_index = stacks
                .iter()
                .position(|stack| stack.id == movement.from)
                .unwrap();
            let to_index = stacks
                .iter()
                .position(|stack| stack.id == movement.to)
                .unwrap();

            let mut to_push: Vec<char> = vec![];

            for _ in 0..movement.count {
                let from = &mut stacks[from_index];

                let el = from.pop().unwrap();

                to_push.push(el);
            }

            let to = &mut stacks[to_index];
            if keep_order {
                for el in to_push.iter().rev() {
                    to.push(el.to_owned());
                }
            } else {
                for el in to_push.iter() {
                    to.push(el.to_owned());
                }
            }
        }
    }
}

#[derive(Debug)]
struct Stack {
    id: u8,
    crates: Vec<char>,
}

impl Stack {
    pub fn new(id: u8) -> Self {
        Stack {
            id,
            crates: Vec::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.crates.push(ch)
    }

    pub fn pop(&mut self) -> Option<char> {
        self.crates.pop()
    }

    pub fn top(&self) -> Option<&char> {
        self.crates.last()
    }
}

#[derive(Debug)]
struct Movement {
    from: u8,
    to: u8,
    count: u8,
}

impl Movement {
    fn new(description: String) -> Movement {
        let splitted: Vec<&str> = description.split(" ").collect();
        Movement {
            count: splitted[1].to_string().parse().unwrap(),
            from: splitted[3].to_string().parse().unwrap(),
            to: splitted[5].to_string().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_push_pop() {
        let mut stack = super::Stack::new(1);
        stack.push('a');
        stack.push('b');
        stack.push('c');
        assert_eq!(stack.pop(), Some('c'));
        assert_eq!(stack.pop(), Some('b'));
        assert_eq!(stack.pop(), Some('a'));
        assert_eq!(stack.pop(), None);
    }
}
