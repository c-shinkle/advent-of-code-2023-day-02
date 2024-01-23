use day_2::{input::INPUT, sum_of_winning_games};

fn main() {
    println!("{}", INPUT.lines().map(sum_of_winning_games).sum::<u32>());
}
