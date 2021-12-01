
fn get_input() -> Vec<i64> {
    include_str!("input").lines().map(|line| line.parse::<i64>().unwrap()).collect()
}

fn part_1(lines: &[i64]) -> i64 {
    lines.windows(2).filter(|&w| w[1]>w[0]).count() as i64
}

fn part_2(lines: &[i64]) -> i64 {
    let windows_of_3: Vec<i64> = lines.windows(3).map(|w| w.iter().sum()).collect();
    part_1(&windows_of_3)
}

fn main() {
    let lines = get_input();
    println!("part1 is {:?}", part_1(&lines));
    println!("part2 is {:?}", part_2(&lines));
}
