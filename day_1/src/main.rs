use std::fs::read_to_string;
use std::io::{Error, ErrorKind};

fn read_input_file(filename: &str) -> Result<String, Error> {
    read_to_string(filename)
}

fn handle_file_error(e: Error, filename: &str) -> ! {
    match e.kind() {
        ErrorKind::NotFound => {
            eprintln!("Error: File '{}' not found", filename);
            eprintln!("Make sure you're running from the correct directory");
        },
        ErrorKind::PermissionDenied => {
            eprintln!("Error: Permission denied reading '{}'", filename);
            eprintln!("Check file permissions");
        },
        _ => {
            eprintln!("Error reading file: {}", e);
        }
    }
    std::process::exit(1);
}

fn parse_rotation(line: &str) -> Result<(char, i32), String> {
    if line.len() < 2 {
        return Err(format!("Line too short: '{}'", line));
    }
    
    let direction = match line.chars().next() {
        Some(d @ 'L') | Some(d @ 'R') => d,
        _ => return Err(format!("Invalid direction in '{}'", line)),
    };
    
    let distance = match line[1..].parse::<i32>() {
        Ok(d) => d,
        Err(e) => return Err(format!("Invalid number in '{}': {}", line, e)),
    };
    
    Ok((direction, distance))
}

// ============================================================================
// PART 1: Count zeros only at the END of each rotation
// ============================================================================

// fn apply_rotation(position: i32, direction: char, distance: i32) -> i32 {
//     match direction {
//         'R' => (position + distance) % 100,
//         'L' => (position - distance + 100) % 100,
//         _ => position,
//     }
// }
//
// fn solve_puzzle(input: &str) -> u32 {
//     let mut position = 50;
//     let mut count = 0;
//     
//     for line in input.lines() {
//         let line = line.trim();
//     
//         if line.is_empty() {
//             continue;
//         }
//         
//         match parse_rotation(line) {
//             Ok((direction, distance)) => {
//                 position = apply_rotation(position, direction, distance);
//                 if position == 0 {
//                     count += 1;
//                 }
//             },
//             Err(e) => {
//                 eprintln!("Warning: Invalid rotation '{}': {}", line, e);
//                 continue;
//             }
//         }
//     }
//     
//     count
// }

// ============================================================================
// PART 2: Count zeros DURING each rotation (every click/step)
// ============================================================================

fn apply_rotation_with_zero_count(position: i32, direction: char, distance: i32) -> (i32, u32) {
    let mut current = position;
    let mut zero_count = 0;
    
    for _ in 0..distance {
        match direction {
            'R' => {
                current = (current + 1) % 100;
            },
            'L' => {
                current = (current - 1 + 100) % 100;
            },
            _ => break,
        }
        
        if current == 0 {
            zero_count += 1;
        }
    }
    
    (current, zero_count)
}

fn solve_puzzle(input: &str) -> u32 {
    let mut position = 50;
    let mut count = 0;
    
    for line in input.lines() {
        let line = line.trim();
    
        if line.is_empty() {
            continue;
        }
        
        match parse_rotation(line) {
            Ok((direction, distance)) => {
                let (new_position, zeros_during_rotation) = 
                    apply_rotation_with_zero_count(position, direction, distance);
                position = new_position;
                count += zeros_during_rotation;
            },
            Err(e) => {
                eprintln!("Warning: Invalid rotation '{}': {}", line, e);
                continue;
            }
        }
    }
    
    count
}

fn main() {
    let filename = "input.txt";
    
    let input = match read_input_file(filename) {
        Ok(input) => input,
        Err(e) => {
            handle_file_error(e, filename);
        }
    };
    
    let password = solve_puzzle(&input);
    println!("Password: {}", password);
}