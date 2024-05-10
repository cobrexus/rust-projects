use std::{collections::HashMap, io};

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    populate(&mut nums);
    println!("median: {}", median(&nums));
    println!("mode: {}", mode(&nums));
}

fn populate(nums: &mut Vec<i32>) {
    let mut input = String::new();
    println!("Input numbers separated by spaces:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    for term in input.split_whitespace() {
        match term.parse::<i32>() {
            Ok(num) => nums.push(num),
            Err(_) => continue,
        }
    }

    if nums.is_empty() {
        panic!("No numbers were entered");
    }
}

fn median(nums: &[i32]) -> i32 {
    let mut temp = nums.to_owned();
    temp.sort_unstable();
    *temp.get(temp.len() / 2).unwrap()
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();

    for num in nums {
        match map.get(num) {
            Some(occurrences) => map.insert(*num, occurrences + 1),
            None => map.insert(*num, 1),
        };
    }

    let mut map_vec = map.iter().collect::<Vec<_>>();
    map_vec.sort_unstable_by(|a, b| a.1.cmp(b.1).reverse());
    *map_vec[0].0
}
