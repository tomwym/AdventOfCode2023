use crate::util;

fn get_numeric_indices(input: &String) -> Vec<usize> {
    let mut buffer: Vec<usize> = Vec::new();
    for (i, c) in input.char_indices() {
        if c.is_ascii_digit() {
            buffer.push(i)
        }
    }
    buffer
}

fn sum_for_element(input: &String) -> u32 {
    let indices = get_numeric_indices(input);
    if indices.len() == 0 {
        return 0;
    }
    let first = (input.as_bytes()[*indices.first().unwrap()] as char).to_digit(10).unwrap();
    let last = (input.as_bytes()[*indices.last().unwrap()] as char).to_digit(10).unwrap();
    first*10+last
}

pub fn part1() -> u32 {
    let vec = util::input_as_vector_string(util::file_path("01"), false);
    let mut sum: u32 = 0;
    for entry in &vec {
        sum = sum+sum_for_element(entry);
    }
    // println!("{sum}");
    sum
}

fn replace_number_strings(input: String) -> String
{
    let numbers = vec![
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    // earliest_index along the input string axis
    let mut earliest_index: Vec<Option<usize>> = vec![None; 10];
    for (i, key) in numbers.iter().enumerate() {
        match input.find(key) {
            Some(val) => earliest_index[i] = Some(val),
            _ => (),
        }
    }
    let mut min_index: (usize, usize) = (0, std::usize::MAX);
    for (numbers_index, input_index) in earliest_index.iter().enumerate() {
        match input_index {
            Some(val) => if val < &min_index.1 {
                min_index.0 = numbers_index;
                min_index.1 = *val;
            },
            None => (),
        }
    }
    if min_index.1 < std::usize::MAX {
        let mut input = input;
        input.replace_range(
            // range: number index to length of number as string
            (min_index.1)..(min_index.1+numbers[min_index.0].len()),
            // replace_with: numerical value with first and last letter i.e. "nine" => "n9e"
            // courtesy of https://www.reddit.com/r/adventofcode/comments/1884fpl/2023_day_1for_those_who_stuck_on_part_2/kbiywz6/
            format!("{}{}{}",
                &numbers[min_index.0].chars().next().unwrap(),
                &min_index.0.to_string().as_str(),
                &numbers[min_index.0].chars().last().unwrap()).as_str()
        );
        replace_number_strings(input)
    }
    else {
        input
    }
}


pub fn part2() -> u32 {

    let vec = util::input_as_vector_string(util::file_path("01"), false);
    let mut sum: u32 = 0;
    for entry in &vec {
        sum = sum+sum_for_element(&replace_number_strings(entry.clone()));
    }
    // println!("{sum}");
    sum
}

