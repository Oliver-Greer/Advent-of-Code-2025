use std::fs;

// ----------------
// PART ONE:
// ----------------

fn largest_jolt(line: &str) -> String {
    
    let mut largest = line.chars().nth(0).unwrap().to_digit(10);
    let mut index_of = 0;
    for i in 1..line.len() - 1 {
        if line.chars().nth(i).unwrap().to_digit(10) > largest {
            largest = line.chars().nth(i).unwrap().to_digit(10);
            index_of = i;
        }
    }
    let mut s_largest = line.chars().nth(index_of + 1).unwrap().to_digit(10);
    for i in index_of + 2..line.len() {
        if line.chars().nth(i).unwrap().to_digit(10) > s_largest {
            s_largest = line.chars().nth(i).unwrap().to_digit(10);
        }
    }
    let s: String = format!("{}{}", largest.unwrap(), s_largest.unwrap());
    s
}

fn main() {
    
    let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
    let mut sum: usize = 0;

    for line in contents.lines() {
        sum += largest_jolt(line).parse::<usize>().expect("cannot convert");
    }

    println!("{sum}");
}