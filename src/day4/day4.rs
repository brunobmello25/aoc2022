use crate::Day;

pub struct Day4 {}

impl Day for Day4 {
    fn run(&self, part: &'static str) {
        match part {
            "part1" => self.run_part_1(),
            "part2" => self.run_part_2(),
            _ => unreachable!(),
        }
    }
}

impl Day4 {
    pub fn new() -> Self {
        Day4 {}
    }

    #[allow(dead_code)]
    fn run_part_1(&self) {
        let filename = "inputs/day4.txt";
        let input = self.read_input(filename);

        let pairs = Pair::input_to_pairs(input.as_str());

        let result = Pair::find_fully_overlapping_pairs(pairs).len();

        println!("result part 1: {result}");
    }

    #[allow(dead_code)]
    fn run_part_2(&self) {
        let filename = "inputs/day4.txt";
        let input = self.read_input(filename);

        let pairs = Pair::input_to_pairs(input.as_str());
        let pairs: Vec<Pair> = pairs
            .into_iter()
            .map(|mut pair| {
                pair.order_ranges();
                pair
            })
            .collect();

        let result = Pair::find_overlapping_pairs(pairs).len();

        println!("result part 2: {result}");
    }

    fn read_input(&self, filename: &str) -> String {
        std::fs::read_to_string(filename).expect("failed to read input content")
    }
}

#[derive(Debug, PartialEq)]
struct Pair {
    first: (usize, usize),
    second: (usize, usize),
}

impl Pair {
    fn input_to_pairs(input: &str) -> Vec<Pair> {
        input
            .trim()
            .split("\n")
            .map(|line| {
                let mut iter = line.split(",");

                (iter.next().unwrap(), iter.next().unwrap())
            })
            .map(|tuple| {
                let mut first = tuple.0.split("-");
                let mut second = tuple.1.split("-");

                Pair {
                    first: (
                        first.next().unwrap().parse().unwrap(),
                        first.next().unwrap().parse().unwrap(),
                    ),
                    second: (
                        second.next().unwrap().parse().unwrap(),
                        second.next().unwrap().parse().unwrap(),
                    ),
                }
            })
            .collect()
    }

    fn find_fully_overlapping_pairs(pairs: Vec<Pair>) -> Vec<Pair> {
        pairs
            .into_iter()
            .filter(|pair| {
                let bigger = pair.bigger_range();

                match bigger {
                    "first" => {
                        return pair.first.0 <= pair.second.0 && pair.first.1 >= pair.second.1
                    }
                    "second" => {
                        return pair.second.0 <= pair.first.0 && pair.second.1 >= pair.first.1
                    }
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    fn bigger_range(&self) -> &str {
        if self.first.1 - self.first.0 > self.second.1 - self.second.0 {
            return "first";
        }

        return "second";
    }

    fn order_ranges(&mut self) {
        if self.first.0 > self.second.0 {
            let tmp = self.first;
            self.first = self.second;
            self.second = tmp;
        }
    }

    fn find_overlapping_pairs(pairs: Vec<Pair>) -> Vec<Pair> {
        pairs
            .into_iter()
            .filter(|pair| pair.second.0 <= pair.first.1)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Pair;

    #[test]
    fn test_input_to_pairs() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n";

        let result = Pair::input_to_pairs(input);

        assert_eq!(
            result[0],
            (Pair {
                first: (2, 4),
                second: (6, 8)
            })
        );
        assert_eq!(
            result[1],
            (Pair {
                first: (2, 3),
                second: (4, 5)
            })
        );
        assert_eq!(
            result[2],
            (Pair {
                first: (5, 7),
                second: (7, 9)
            })
        );
    }

    #[test]
    fn test_bigger_range() {
        let inputs = vec![
            Pair {
                first: (0, 2),
                second: (0, 4),
            },
            Pair {
                first: (0, 7),
                second: (3, 8),
            },
            Pair {
                first: (0, 7),
                second: (3, 5),
            },
        ];
        assert_eq!(inputs[0].bigger_range(), "second");
        assert_eq!(inputs[1].bigger_range(), "first");
        assert_eq!(inputs[2].bigger_range(), "first");
    }

    #[test]
    fn test_find_fully_overlapping_pairs() {
        let input = vec![
            Pair {
                first: (2, 4),
                second: (6, 8),
            },
            Pair {
                first: (5, 7),
                second: (7, 9),
            },
            Pair {
                first: (2, 8),
                second: (3, 7),
            },
        ];

        let result = Pair::find_fully_overlapping_pairs(input);

        assert_eq!(result.len(), 1);
        assert_eq!(
            result,
            vec![Pair {
                first: (2, 8),
                second: (3, 7)
            }]
        )
    }
}
