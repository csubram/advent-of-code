use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_gamma_rate() -> i32 {
    let input_vec = read_input_into_vector();
    let mut gamma:i32 = 0;

    for index in (0..11).rev() {
        gamma += most_common_nth_bit(&input_vec, &index);
        gamma <<= 1;
    }
    
    gamma >> 1
}

fn get_file_reader() -> BufReader<File>{
    let file = File::open("src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    reader
}

fn convert_line_to_num(line: std::result::Result<std::string::String, std::io::Error>) -> i32 {
    let bin_str: String = line.expect("Could not read line");
    let decimal_value: i32 = i32::from_str_radix(&bin_str, 2).unwrap();
    decimal_value
}

fn read_input_into_vector() -> Vec<i32> {
    let input_vec = get_file_reader()
                        .lines()
                        .map(|l| convert_line_to_num(l))
                        .collect();
    input_vec
}

fn count_bit_frequency_nth_position(values: &Vec<i32>, n: &i32) -> i32 {
    let mut count = 0;
    for item in values {
        if (item & (1 << n)) > 0 {
            count += 1;
        } else {
            count -= 1;
        }
    }

    count
}

fn most_common_nth_bit(values: &Vec<i32>, n: &i32) -> i32 {
    let count = count_bit_frequency_nth_position(&values, &n);
    if count >= 0 {1} else {0}
}

fn least_common_nth_bit(values: &Vec<i32>, n: &i32) -> i32 {
    let count = count_bit_frequency_nth_position(&values, &n);
    if count >= 0 {0} else {1}
}

fn get_og_co_rating(input: &Vec<i32>, predicate: &dyn Fn(&Vec<i32>, &i32) -> i32) -> i32 {
    let mut remaining_results = input.clone();

    let mut index = 11; // hardcoded the length of the input bc I'm lazy
    while remaining_results.len() > 1 {
        let target_bit = predicate(&remaining_results, &index);
        remaining_results = remaining_results
                            .into_iter()
                            .filter(|x| (x & (1 << index)) == (target_bit << index))
                            .collect();
        index -= 1;
    }

    remaining_results[0]
}

fn main() {
    let mask: i32 = 0b1111_1111_1111;
    let gamma: i32 = get_gamma_rate();
    let epsilon: i32 = gamma ^ mask;

    println!("Power consumption is: {}", gamma * epsilon);

    let input_vec = read_input_into_vector();
    let oxygen_generator_rating: i32 = get_og_co_rating(&input_vec, &most_common_nth_bit);
    let co2_scrubber_rating: i32 = get_og_co_rating(&input_vec, &least_common_nth_bit);

    println!("Life support rating is: {}", oxygen_generator_rating * co2_scrubber_rating);
}
