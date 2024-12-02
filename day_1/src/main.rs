use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    let mut sum = 0;
    for (i, val) in file.split_whitespace().enumerate() {
        if i % 2 == 0 {
            first_list.push(val.parse::<i32>().unwrap())
        } else {
            second_list.push(val.parse::<i32>().unwrap())
        }
    }
    first_list.sort();
    second_list.sort();
    for (val_one, val_two) in first_list.iter().zip(second_list) {
        println!("{}, {}", val_one, val_two);
        sum += (val_one - val_two).abs();
    }
    println!("{}", sum.abs());
}
