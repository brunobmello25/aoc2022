use day6::day6::Day6;

mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

trait Day {
    fn run(&self, part: &'static str);
}

fn main() {
    let day = Day6::new();

    day.run("part1");
}
