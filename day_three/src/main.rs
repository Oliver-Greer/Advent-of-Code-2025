use std::fs;

// ----------------
// PART ONE:
// ----------------

// fn largest_jolt(line: &str) -> String {
    
//     let mut largest = line.chars().nth(0).unwrap().to_digit(10);
//     let mut index_of = 0;
//     for i in 1..line.len() - 1 {
//         if line.chars().nth(i).unwrap().to_digit(10) > largest {
//             largest = line.chars().nth(i).unwrap().to_digit(10);
//             index_of = i;
//         }
//     }
//     let mut s_largest = line.chars().nth(index_of + 1).unwrap().to_digit(10);
//     for i in index_of + 2..line.len() {
//         if line.chars().nth(i).unwrap().to_digit(10) > s_largest {
//             s_largest = line.chars().nth(i).unwrap().to_digit(10);
//         }
//     }
//     let s: String = format!("{}{}", largest.unwrap(), s_largest.unwrap());
//     s
// }

// fn main() {
    
//     let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
//     let mut sum: usize = 0;

//     for line in contents.lines() {
//         sum += largest_jolt(line).parse::<usize>().expect("cannot convert");
//     }

//     println!("{sum}");
// }

// ----------------
// PART TWO:
// ----------------

fn largest_jolt(line: &mut String, answer: &mut String, start: usize, end: usize) -> String {
    
    if answer.len() == 12 {
        return answer.chars().collect();
    }
    
    let mut largest = line.chars().nth(start).unwrap().to_digit(10);
    let mut index_of = start;
    for i in start + 1..line.len() - end + 1 {
        if line.chars().nth(i).unwrap().to_digit(10) > largest {
            largest = line.chars().nth(i).unwrap().to_digit(10);
            index_of = i;
        }
    }
    *answer += &line.chars().nth(index_of).unwrap().to_string();
    largest_jolt(line, answer, index_of + 1, end - 1)
}

fn main() {
    
    let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
    let mut sum: usize = 0;
    let end = 12;

    for line in contents.lines() {
        sum += largest_jolt(&mut line.to_string(), &mut "".to_string(), 0, end).parse::<usize>().expect("cannot convert");
    }

    println!("{sum}");
}