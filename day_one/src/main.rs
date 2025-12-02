use std::fs;

// ----------------
// PART ONE:
// ----------------
// fn main() {
//     let mut start_val = 50;
//     let total_clicks = 100;
//     let mut zeros = 0;

//     let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
//     for line in contents.lines() {
//         let direction = line.chars().nth(0);
//         let number: i32 = line[1..].parse().unwrap();
//         let mut result = start_val;
//         if matches!(direction, Some('R')) {
//             result = result + number;
//             result = result % total_clicks;
//         } else {
//             result = result - number + total_clicks;
//             result = result % total_clicks;
//         }
//         if result == 0 {
//             zeros += 1;
//         }
//         start_val = result;
//     }

//     println!("{zeros}");
// }


// ----------------
// PART TWO:
// ----------------
fn main() {
    let mut start_val = 50;
    let total_clicks = 100;
    let mut zeros = 0;

    let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
    for line in contents.lines() {
        let direction = line.chars().nth(0);
        let number: i32 = line[1..].parse().unwrap();
        let mut result = start_val;
        for _ in 0..number {
            if matches!(direction, Some('R')) {
            result = result + 1;
            result = result % total_clicks;
            } else {
                result = result - 1 + total_clicks;
                result = result % total_clicks;
            }
            if result == 0 {
                zeros += 1;
            }
        }
        start_val = result;
    }

    println!("{zeros}");
}