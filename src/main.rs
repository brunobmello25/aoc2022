use day4::day4::Day4;

mod day2;
mod day3;
mod day4;

trait Day {
    fn run(&self, part: &'static str);
}

fn main() {
    let day = Day4::new();

    day.run("part2");
}
