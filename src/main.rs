use day2::day2::Day2;

mod day2;

trait Day {
    fn run(&self, part: &'static str);
}

fn main() {
    let day2 = Day2::new();

    day2.run("part1");
}
