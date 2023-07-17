use day3::day3::Day3;

mod day2;
mod day3;

trait Day {
    fn run(&self, part: &'static str);
}

fn main() {
    let day = Day3::new();

    day.run("part2");
}
