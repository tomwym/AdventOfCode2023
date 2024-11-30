const SYMBOLS: &str = "@#$%&*-=+/";
const GEAR_SYMBOL: &str = "*";

use crate::util;

fn find_numeric_intervals(field: &Vec<String>) -> Vec<Vec<(u32, u32)>> {
    let mut buffer: Vec<Vec<(u32, u32)>> = Vec::new();
    for row in field {
        let mut row_buff: Vec<(u32, u32)> = Vec::new();
        let mut in_number = false;
        let mut start: u32 = 0;
        for (i, char) in row.chars().enumerate() {
            let is_digit = char.is_ascii_digit();
            if in_number && is_digit {}
            else if !in_number && is_digit {
                in_number = true;
                start = u32::try_from(i).unwrap();
            }
            else if in_number && !is_digit {
                in_number = false;
                row_buff.push((start, u32::try_from(i-1).unwrap()));
            }
        }
        // end bound
        if (in_number) {
            row_buff.push((start, u32::try_from(row.len()-1).unwrap()));
        }
        buffer.push(row_buff);
    }
    buffer
}

fn find_symbols(field: &Vec<String>, symbols: &str) -> Vec<(u32, u32)> {
    field
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
            .enumerate()
            .filter_map( move |(j, c)| {
                if symbols.contains(c) {
                    Some((i as u32, j as u32))
                } else {
                    None
                } 
            })
        })
        .collect()
}

fn find_j_in_intervals(j: u32, intervals: &Vec<(u32, u32)>) -> Option<usize> {
    intervals.iter().position(|interval| if interval.0 <=j && j <= interval.1 {true} else {false})
}

fn get_value_and_remove_interval(j: u32, row: &String, intervals: &mut Vec<(u32, u32)>) -> u32 {
    match find_j_in_intervals(j, &intervals) {
        Some(interval_index) => {
            let interval = &intervals[interval_index];
            // extract interval as int and add to row sum
            let as_int = row[(interval.0 as usize)..((interval.1+1) as usize)].parse::<u32>().unwrap();
            // remove interval_index from intervals
            intervals.remove(interval_index);
            // println!("{as_int}");
            as_int
        },
        None => 0,
    }
}

fn mutable_look_at(i: i32, j: i32, field: &Vec<String>, number_locations: &mut Vec<Vec<(u32, u32)>>) -> u32 {
    let m = field.len();
    let n = field[0].len();

    if 0 <= i && i+1 <= m as i32 &&
       0 <= j && j+1 <= n as i32 {
        let intervals = &mut number_locations[(i) as usize];
        let row = &field[(i) as usize];
        get_value_and_remove_interval(j as u32, row, intervals)
    } else {
        0
    }
}

fn look_around(field: &Vec<String>, number_locations: &mut Vec<Vec<(u32, u32)>>, symbol: &(u32, u32)) -> u32 {
    let i = symbol.0 as i32;
    let j = symbol.1 as i32;

    let mut sum: u32 = 0;
    sum += mutable_look_at(i-1, j-1, field, number_locations);
    sum += mutable_look_at(i-1, j, field, number_locations);
    sum += mutable_look_at(i-1, j+1, field, number_locations);
    sum += mutable_look_at(i, j-1, field, number_locations);
    sum += mutable_look_at(i, j+1, field, number_locations);
    sum += mutable_look_at(i+1, j-1, field, number_locations);
    sum += mutable_look_at(i+1, j, field, number_locations);
    sum += mutable_look_at(i+1, j+1, field, number_locations);

    sum
}

pub fn part1() -> u32 {
    let field: Vec<String> = util::input_as_vector_string(util::file_path("03"), false);
    let mut sum: u32 = 0;
    let mut interval_locations = find_numeric_intervals(&field);
    let symbol_locations = find_symbols(&field, SYMBOLS);
    for symbol in &symbol_locations {
        sum += look_around(&field, &mut interval_locations, &symbol);
    }
    println!("{sum}");
    sum
}

fn near_gear() {

}

fn multiply_adjacent(field: &Vec<String>, number_locations: &mut Vec<Vec<(u32, u32)>>, symbol: &(u32, u32)) -> u32 {
    let i = symbol.0 as i32;
    let j = symbol.1 as i32;

    let mut sum: u32 = 0;
    sum += near_gear(i-1, j-1, field, number_locations);
    sum += near_gear(i-1, j, field, number_locations);
    sum += near_gear(i-1, j+1, field, number_locations);
    sum += near_gear(i, j-1, field, number_locations);
    sum += near_gear(i, j+1, field, number_locations);
    sum += near_gear(i+1, j-1, field, number_locations);
    sum += near_gear(i+1, j, field, number_locations);
    sum += near_gear(i+1, j+1, field, number_locations);

}

pub fn part2() -> u32 {
    let field: Vec<String> = util::input_as_vector_string(util::file_path("test"), false);
    let mut sum: u32 = 0;
    let mut interval_locations = find_numeric_intervals(&field);
    let symbol_locations = find_symbols(&field, GEAR_SYMBOL);
    for symbol in &symbol_locations {
        sum += look_around(&field, &mut interval_locations, &symbol);
    }
    println!("{sum}");
    sum
}

