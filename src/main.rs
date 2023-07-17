mod day2;

trait Day {
    fn run(&self, part: &'static str);

    fn run_part_1(&self);

    fn run_part_2(&self);
}

fn main() {
    let day2 = day2::Day2::new();

    day2.run("part2")
}
