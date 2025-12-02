use std::fs;

// ----------------
// PART ONE:
// ----------------

// fn is_invalid(num: &i64) -> bool {

//     let num_str: String = num.to_string();
//     if num_str.len() % 2 != 0 {
//         return false;
//     }

//     let slice_len = num_str.len() / 2;
//     let init_slice = &num_str[..slice_len];
//     if init_slice == &num_str[slice_len..num_str.len()] {
//         return true;
//     }

//     return false;
// }

// fn main() {
    
//     let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
//     let parts = contents.split(',');
//     let ranges: Vec<&str> = parts.collect();

//     let mut sum = 0;

//     for range in ranges {
//         let s_e: Vec<&str> = range.split('-').collect();
//         let start: i64 = s_e[0].parse().unwrap();
//         let end: i64 = s_e[1].parse().unwrap();

//         for num in start..end+1 {
//             if is_invalid(&num) {
//                 sum += num;
//             }
//         }
//     }

//     println!("{sum}");
// }


// ----------------
// PART TWO:
// ----------------

fn is_invalid(num: &i64) -> bool {

    let num_str: String = num.to_string();
    for slice_len in 1..num_str.len() {
        if num_str.len() % slice_len != 0 {
            continue;
        } else {
            let init_slice = &num_str[..slice_len];
            for slice in 1..num_str.len() / slice_len {
                if init_slice != &num_str[(slice_len * slice)..slice_len * (slice + 1)] {
                    break;
                }
                if slice == num_str.len() / slice_len - 1 {
                    return true;
                }
            }
        }
    }

    return false;
}
fn main() {
    
    let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
    let parts = contents.split(',');
    let ranges: Vec<&str> = parts.collect();

    let mut sum = 0;

    for range in ranges {
        let s_e: Vec<&str> = range.split('-').collect();
        let start: i64 = s_e[0].parse().unwrap();
        let end: i64 = s_e[1].parse().unwrap();

        for num in start..end+1 {
            if is_invalid(&num) {
                sum += num;
            }
        }
    }

    println!("{sum}");
}