use std::{fs::File, io, io::BufRead, path::Path};

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file(path: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(lines_iter) = read_lines(path) {
        for line in lines_iter {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
    }
    lines
}

fn parse_input(lines: Vec<String>) -> Vec<Vec<u32>> {
    // Create a new vector to store the parsed input
    let mut parsed_input = Vec::new();

    // Iterate over the lines
    let mut current_elf_calories = Vec::new();
    for line in lines {
        if let Ok(calories) = line.parse::<u32>() {
            current_elf_calories.push(calories);
        } else {
            parsed_input.push(current_elf_calories);
            current_elf_calories = Vec::new();
        }
    }
    // Return the parsed input
    parsed_input
}

fn get_total_calories(elf_calories: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut total_calories = Vec::new();
    for elf in elf_calories {
        let sum = elf.iter().sum();
        total_calories.push(sum);
    }
    total_calories
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let total_calories = get_total_calories(input);
    *total_calories.iter().max().unwrap() // notice the dereference
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut total_calories = get_total_calories(input);
    // sort array from highest to lowest
    total_calories.sort();
    total_calories.reverse();
    // get the first 3 elements
    let top_three_calories = &total_calories[0..3];
    // sum the elements
    top_three_calories.iter().sum()
}

fn main() {
    let input = read_file("days/day-01/input.txt");
    let parsed_input = parse_input(input);

    let part1_answer = part1(&parsed_input);
    println!("Part 1: {}", part1_answer);

    let part2_answer = part2(&parsed_input);
    println!("Part 2: {}", part2_answer);
}
