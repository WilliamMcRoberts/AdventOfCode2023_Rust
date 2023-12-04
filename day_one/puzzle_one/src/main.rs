use std::fs::read_to_string;

fn main() {
    let lines = read_to_string("../aoc_day_1").expect("No File Found");

    let output = lines
        .lines()
        .map(|l| {
            let mut filtered_map = l.chars().filter_map(|character| character.to_digit(10));

            let first = filtered_map.next().expect("This should be a number");
            let last = filtered_map.last();

            match last {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("This should be a number")
        })
        .sum::<u32>();

    println!("Output: {}", output);
}
