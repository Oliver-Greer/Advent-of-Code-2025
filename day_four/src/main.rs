use std::fs;

// ----------------
// PART ONE:
// ----------------

// fn can_access(content: &Vec<Vec<char>>, x: i64, y: i64) -> bool {
//     let directions: Vec<(i64, i64)> = vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,-1), (-1,1), (1,-1)];

//     let mut total = 0;
//     for dir in directions {
//         if x+dir.0 >= 0 && y+dir.1 >= 0 && x+dir.0 <= (content.len() - 1)  as i64 && y+dir.1 <= (content[0].len() - 1) as i64 {
//             if content[(x+dir.0) as usize][(y+dir.1) as usize] == '@' {
//                 total += 1;
//             }
//         }
//     }

//     if total < 4 {
//         return true;
//     }

//     false
// }

// fn main() {
    
//     let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
//     let mut sum: usize = 0;
//     let mut content: Vec<Vec<char>> = Vec::new();

//     for line in contents.lines() {
//         content.push(line.chars().collect());
//     }

//     for x in 0..content.len() {
//         for y in 0..content[0].len() {
//             if content[x][y] == '@' {
//                 if can_access(&content, x as i64, y as i64) {
//                     sum += 1;
//                 }
//             }
//         }
//     }

//     println!("{sum}");
// }


// ----------------
// PART TWO:
// ----------------

fn can_access(content: &Vec<Vec<char>>, x: i64, y: i64) -> bool {
    let directions: Vec<(i64, i64)> = vec![(1,0), (0,1), (-1,0), (0,-1), (1,1), (-1,-1), (-1,1), (1,-1)];

    let mut total = 0;
    for dir in directions {
        if x+dir.0 >= 0 && y+dir.1 >= 0 && x+dir.0 <= (content.len() - 1)  as i64 && y+dir.1 <= (content[0].len() - 1) as i64 {
            if content[(x+dir.0) as usize][(y+dir.1) as usize] == '@' {
                total += 1;
            }
        }
    }

    if total < 4 {
        return true;
    }

    false
}

fn recurse(content: &mut Vec<Vec<char>>, sum: usize) -> usize {

    let mut tot = 0;
    for x in 0..content.len() {
        for y in 0..content[0].len() {
            if content[x][y] == '@' {
                if can_access(&content, x as i64, y as i64) {
                    content[x][y] = '.';
                    tot += 1;
                }
            }
        }
    }

    if tot == 0 {
        return sum;
    }

    recurse(content, sum + tot)
}

fn main() {
    
    let contents = fs::read_to_string("input.txt").expect("Does file not exist?");
    let mut content: Vec<Vec<char>> = Vec::new();

    for line in contents.lines() {
        content.push(line.chars().collect());
    }

    let sum = recurse(&mut content, 0);

    println!("{sum}");
}