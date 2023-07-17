use std::collections::HashMap;

use crate::Day;

pub struct Day3 {}

impl Day for Day3 {
    fn run(&self, part: &'static str) {
        match part {
            "part1" => self.run_part_1(),
            "part2" => self.run_part_2(),
            _ => unreachable!(),
        }
    }
}

impl Day3 {
    pub fn new() -> Self {
        Day3 {}
    }

    #[allow(dead_code)]
    fn run_part_1(&self) {
        let filename = "inputs/day3.txt";
        let input = std::fs::read_to_string(filename).expect("failed to read input content");

        let rucksacks: Vec<_> = input
            .trim()
            .split("\n")
            .map(|line| Rucksack::new(line.to_string()))
            .collect();

        let sum_weights = rucksacks
            .iter()
            .map(|rucksack| rucksack.find_common_in_halves().unwrap())
            .map(|item| Rucksack::get_item_priority(item))
            .reduce(|acc, el| acc + el)
            .unwrap();

        println!("sum of weights: {}", sum_weights);
    }

    #[allow(dead_code)]
    fn run_part_2(&self) {
        let filename = "inputs/day3.txt";
        let input = std::fs::read_to_string(filename)
            .expect("failed to read input content")
            .trim()
            .to_string();

        let groups = Group::groups_from_input(input);

        let repeating_chars = groups
            .iter()
            .map(|group| group.get_repeating_char().unwrap())
            .map(|ch| Rucksack::get_item_priority(ch))
            .reduce(|acc, el| acc + el)
            .unwrap();

        println!("repeating chars: {:?}", repeating_chars);
    }
}

struct Group {
    first: Rucksack,
    second: Rucksack,
    third: Rucksack,
}

impl Group {
    fn new(first: Rucksack, second: Rucksack, third: Rucksack) -> Self {
        Group {
            first,
            second,
            third,
        }
    }

    fn groups_from_input(input: String) -> Vec<Self> {
        let mut groups = Vec::new();
        let mut lines = input.trim().split("\n");

        while let Some(first) = lines.next() {
            let second = lines.next().unwrap();
            let third = lines.next().unwrap();

            groups.push(Group::new(
                Rucksack::new(first.to_string()),
                Rucksack::new(second.to_string()),
                Rucksack::new(third.to_string()),
            ));
        }

        return groups;
    }

    fn get_repeating_char(&self) -> Option<char> {
        let first_full_string = self.first.get_full_string();
        let second_occurrencies = Rucksack::get_occurencies(self.second.get_full_string().as_str());
        let third_occurrencies = Rucksack::get_occurencies(self.third.get_full_string().as_str());

        for ch in first_full_string.chars() {
            if let None = second_occurrencies.get(&ch) {
                continue;
            }

            if let None = third_occurrencies.get(&ch) {
                continue;
            }

            return Some(ch);
        }

        return None;
    }
}

struct Rucksack {
    first_half: String,
    second_half: String,
}

impl Rucksack {
    fn new(input: String) -> Self {
        let index: usize = ((input.len() as f32) / 2.0).ceil() as usize;
        let (first_half, second_half) = input.split_at(index);
        Rucksack {
            first_half: first_half.to_string(),
            second_half: second_half.to_string(),
        }
    }

    fn get_full_string(&self) -> String {
        let mut content = self.first_half.clone();
        content.push_str(self.second_half.as_str());
        return content;
    }

    fn get_occurencies(half: &str) -> HashMap<char, i32> {
        let mut map = HashMap::new();

        for ch in half.chars() {
            if let Some(count) = map.get(&ch) {
                map.insert(ch, count + 1);
            } else {
                map.insert(ch, 1);
            }
        }

        return map;
    }

    fn get_item_priority(item: char) -> u32 {
        match item {
            'a'..='z' => item as u32 - 96,
            'A'..='Z' => item as u32 - 38,
            _ => unreachable!(),
        }
    }

    fn find_common_in_halves(&self) -> Option<char> {
        let occurrencies = Rucksack::get_occurencies(&self.first_half);

        for char in self.second_half.chars() {
            if let Some(_) = occurrencies.get(&char) {
                return Some(char);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::day3::Group;

    use super::Rucksack;

    #[test]
    fn item_priority() {
        let chars = vec!['a', 'z', 'A', 'Z'];
        let expected = vec![1, 26, 27, 52];

        for (i, _) in chars.iter().enumerate() {
            assert_eq!(Rucksack::get_item_priority(chars[i]), expected[i]);
        }
    }

    #[test]
    fn rucksack_input_split() {
        let inputs = vec!["abcdefg", "abcdefgh"];
        let asserts = vec![("abcd", "efg"), ("abcd", "efgh")];

        for (i, input) in inputs.iter().enumerate() {
            let rucksack = Rucksack::new(input.to_string());

            let (first, second) = asserts[i];
            assert_eq!(first, rucksack.first_half);
            assert_eq!(second, rucksack.second_half);
        }
    }

    #[test]
    fn occurrencies() {
        let input = "aFMMfFFhFp";
        let occurrencies = Rucksack::get_occurencies(input);

        assert_eq!(occurrencies.get(&'a').unwrap().to_owned(), 1);
        assert_eq!(occurrencies.get(&'F').unwrap().to_owned(), 4);
        assert_eq!(occurrencies.get(&'f').unwrap().to_owned(), 1);
        assert_eq!(occurrencies.get(&'M').unwrap().to_owned(), 2);
        assert_eq!(occurrencies.get(&'p').unwrap().to_owned(), 1);
        assert_eq!(occurrencies.get(&'h').unwrap().to_owned(), 1);
    }

    #[test]
    fn find_common() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::new(input.to_string());
        let common = rucksack.find_common_in_halves();
        assert_eq!(common.unwrap(), 'p');
    }

    #[test]
    fn group_get_repeating_char() {
        let rucksack_a = Rucksack::new("abcde".to_string());
        let rucksack_b = Rucksack::new("ABCDe".to_string());
        let rucksack_c = Rucksack::new("1234e".to_string());

        let group = Group::new(rucksack_a, rucksack_b, rucksack_c);
        assert_eq!(group.get_repeating_char().unwrap(), 'e')
    }

    #[test]
    fn test_groups_from_input() {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye";
        let groups = Group::groups_from_input(input.to_string());

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].first.get_full_string(), "abcde");
        assert_eq!(groups[0].second.get_full_string(), "fghij");
        assert_eq!(groups[0].third.get_full_string(), "klmno");
        assert_eq!(groups[1].first.get_full_string(), "pqrst");
        assert_eq!(groups[1].second.get_full_string(), "fguij");
        assert_eq!(groups[1].third.get_full_string(), "axcye");
    }
}
