use crate::Day;

pub struct Day2 {}

impl Day for Day2 {
    fn run(&self, part: &'static str) {
        match part {
            "part1" => self.run_part_1(),
            "part2" => self.run_part_2(),
            _ => unreachable!(),
        }
    }

    fn run_part_1(&self) {
        let data = self.parse_input("inputs/day2.txt");

        let result = data
            .iter()
            .map(|play| play.get_play_points())
            .reduce(|acc, el| acc + el)
            .unwrap();

        println!("result: {}", result);
    }

    fn run_part_2(&self) {
        let data = self.parse_input("inputs/day2.txt");

        let result = data
            .iter()
            .map(|play| play.get_points_by_needed_result())
            .reduce(|acc, el| acc + el)
            .unwrap();

        println!("result: {}", result);
    }
}

impl Day2 {
    pub fn new() -> Self {
        Day2 {}
    }

    fn parse_input(&self, filename: &'static str) -> Vec<Play> {
        let input = std::fs::read_to_string(filename).expect("failed to read input content");

        input
            .trim()
            .split("\n")
            .map(|line| {
                let mut chars = line.chars();
                let opponent = chars.next().unwrap();
                let you = chars.last().unwrap();

                Play { opponent, you }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Play {
    opponent: char,
    you: char,
}

impl Play {
    fn get_play_points(&self) -> u32 {
        self.get_selected_shape_score() + self.get_round_result_score()
    }

    fn get_selected_shape_score(&self) -> u32 {
        match self.you {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => unreachable!(),
        }
    }

    fn get_round_result_score(&self) -> u32 {
        match (self.opponent, self.you) {
            ('A', 'X') => 3,
            ('A', 'Y') => 6,
            ('A', 'Z') => 0,
            ('B', 'X') => 0,
            ('B', 'Y') => 3,
            ('B', 'Z') => 6,
            ('C', 'X') => 6,
            ('C', 'Y') => 0,
            ('C', 'Z') => 3,
            (a, b) => {
                println!("{:?} {:?}", a, b);
                unreachable!()
            }
        }
    }

    fn get_points_by_needed_result(&self) -> u32 {
        // 0 if you lost, 3 if the round was a draw, and 6 if you won
        // 1 for Rock, 2 for Paper, and 3 for Scissors
        match (self.opponent, self.you) {
            ('A', 'X') => 0 + 3, // botou pedra: quero perder -> boto tesoura
            ('A', 'Y') => 3 + 1, // botou pedra: quero empatar -> boto pedra
            ('A', 'Z') => 6 + 2, // botou pedra: quero ganhar -> boto papel
            ('B', 'X') => 0 + 1, // botou papel: quero perder -> boto pedra
            ('B', 'Y') => 3 + 2, // botou papel: quero empatar -> boto papel
            ('B', 'Z') => 6 + 3, // botou papel: quero ganhar -> boto tesoura
            ('C', 'X') => 0 + 2, // botou tesoura: quero perder -> boto papel
            ('C', 'Y') => 3 + 3, // botou tesoura: quero empatar -> boto tesoura
            ('C', 'Z') => 6 + 1, // botou tesoura: quero ganhar -> boto pedra
            (_, _) => unreachable!(),
        }
    }
}
