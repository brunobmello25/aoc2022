use crate::Day;

pub struct Day6 {}

impl Day for Day6 {
    fn run(&self, part: &'static str) {
        match part {
            "part1" => self.run_part_1(),
            "part2" => self.run_part_2(),
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
impl Day6 {
    pub fn new() -> Self {
        Day6 {}
    }

    fn run_part_1(&self) {
        let input = self.read_input();

        self.find_index_of_first_different_fours(input);
    }

    fn run_part_2(&self) {}

    fn read_input(&self) -> String {
        std::fs::read_to_string("inputs/day6.txt").expect("Failed to read input")
    }

    fn find_index_of_first_different_fours(&self, input: String) {
        let chars: Vec<char> = input.trim().chars().collect();
        for i in 3..input.len() {
            println!("i: {}", chars.get(i).unwrap());
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
