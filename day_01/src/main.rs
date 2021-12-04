use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_file_reader() -> BufReader<File>{
    let file = File::open("src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
}

fn convert_line_to_num(line: std::result::Result<std::string::String, std::io::Error>) -> u32 {
    let text = line.expect("Could not convert line into text");
    let val: u32 = text.parse().unwrap();
    val
}

fn num_of_descents() -> u32 {
    let mut count: u32 = 0;
    let mut prev_value: u32 = u32::max_value();

    for line in get_file_reader().lines() {
        let val: u32 = convert_line_to_num(line);
        if val > prev_value {
            count += 1;
        }
        prev_value = val;
    }

    count
}

fn sliding_window_descents(window_size: usize) -> u32 {
    let mut count: u32 = 0;
    let mut prev_value: u32 = u32::max_value();

    let mut sliding_window = vec![0; window_size];

    let lines = get_file_reader().lines();
    for (i, line) in lines.enumerate() {
        let val:u32 = convert_line_to_num(line);

        sliding_window[i % window_size] = val;

        if i >= (window_size - 1) {
            let sum: u32 = sliding_window.iter().sum();
            if sum > prev_value {
                count += 1;
            }
            prev_value = sum;
        }
    }

    count
}

fn main() {
    println!("Number of descents: {}", num_of_descents()); //1233

    let window_size: usize = 3;
    println!("Sliding window descents: {}", sliding_window_descents(window_size)); //1275
}
