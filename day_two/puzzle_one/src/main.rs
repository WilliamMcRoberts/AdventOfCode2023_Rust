use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let lines = read_to_string("../aoc_day_2").expect("Could not read file");
    let lines = lines.lines();

    let res = lines.map(|l| {
        let mut segments = l.split(":");
        let game = segments.next();
        let cube_segments = segments
            .next()
            .unwrap()
            .trim()
            .split(";")
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>();


        let Vec<(u32,u32,u32)> =  cube_segments.iter().map(|c| {

            let red: u32 = 0;
            let green: u32 = 0;
            let blue: u32 = 0;

            let cube_split: Vec<&str> = c.trim().split("x").collect();
            let num = cube_split[0].parse::<u32>().unwrap();
            let color = cube_split[1];

        }).collect::<Vec<(u32,u32,u32)>>();
    });

    for r in res {
        println!("{:?}", r);
    }
}
