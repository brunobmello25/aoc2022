use day5::day5::Day5;

mod day2;
mod day3;
mod day4;
mod day5;

trait Day {
    fn run(&self, part: &'static str);
}

fn main() {
    let day = Day5::new();

    day.run("part2");
}
