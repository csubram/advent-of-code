use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Point {
    x: u32,
    y: u32,
    aim: u32,
}

fn get_file_reader() -> BufReader<File>{
    let file = File::open("src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
}

fn get_final_position() -> Point {
    let mut current_position = Point {x: 0, y: 0, aim: 0};

    for line in get_file_reader().lines() {
        let text = line.expect("Could not read input line");
        let instruction: Vec<&str> = text.split_whitespace().collect();

        let direction = instruction[0];
        let magnitude: u32 = instruction[1].parse().expect("Could not parse magnitude");

        match direction {
            "forward" => current_position.x += magnitude,
            "up" => current_position.y -= magnitude,
            "down" => current_position.y += magnitude,
            _ => println!("Unexpected direction?"),
        }
    }

    current_position
}

fn get_final_position_v2() -> Point {
    let mut current_position = Point {x: 0, y: 0, aim: 0};

    for line in get_file_reader().lines() {
        let text = line.expect("Could not read input line");
        let instruction: Vec<&str> = text.split_whitespace().collect();

        let direction = instruction[0];
        let magnitude: u32 = instruction[1].parse().expect("Could not parse magnitude");

        match direction {
            "forward" => { 
                current_position.x += magnitude;
                current_position.y += magnitude * current_position.aim
            },
            "up" => current_position.aim -= magnitude,
            "down" => current_position.aim += magnitude,
            _ => println!("Unexpected direction!!"),
        }
    }

    current_position
}

fn main() {
    let final_position = get_final_position();
    println!("Final x: {}, final y: {}", final_position.x, final_position.y);
    println!("Multiplied together = {}", final_position.x * final_position.y);

    let recalc_final_position = get_final_position_v2();
    println!("Final x: {}, final y: {}", recalc_final_position.x, recalc_final_position.y);
    println!("Multiplied together = {}", recalc_final_position.x * recalc_final_position.y);
}
