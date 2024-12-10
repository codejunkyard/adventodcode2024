use advent_lib::fetch_input;
use dotenv::dotenv;
use std::env;
use std::error::Error;

pub fn solve() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let url = "https://adventofcode.com/2024/day/9/input";
    let session_token = env::var("SESSION_TOKEN").expect("SESSION_TOKEN must be set");

    //let input = fetch_input(url, &session_token)?;
    //let input = "12345"; // 60
    //let input = "2333133121414131402";
    let input = "23331331214141314024893970074";

    let digits: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(10)) // Filter non-digit characters if necessary
        .map(|d| d as u8) // Convert to `u8` for more efficient storage
        .collect();

    //let count_part_1 = get_part_1(&digits);
    //println!("Part 1: Total count: {}", count_part_1);

    let count_part_2 = get_part_2(&digits);
    println!("Part 2: Total count: {}", count_part_2);

    Ok(())
}

fn get_part_1(digits: &[u8]) -> usize {
    // println!("    Original: 12345"); // len: 5
    // println!("Decompressed: 0..111....22222");
    // println!("  Compressed: 022111222......");
    // println!("    Checksum: 60");
    // println!();

    // println!("    Original: 2333133121414131402"); // len: 19
    // println!("Decompressed: 00...111...2...333.44.5555.6666.777.888899");
    // println!("  Compressed: 0099811188827773336446555566..............");
    // println!("    Checksum: 1928");
    // println!();

    let mut sum = 0;
    let mut buffer: Vec<usize> = Vec::new();
    let mut global_index = 0;

    struct Space {
        total: usize,
        free: usize,
    }

    let mut disk_space = Space { total: 0, free: 0 };

    for (index, &value) in digits.iter().enumerate() {
        disk_space.total += value as usize;

        if index % 2 != 0 {
            disk_space.free += value as usize;
        }
    }

    for (index, &value) in digits.iter().rev().enumerate() {
        if index % 2 == 0 {
            for _ in 0..value {
                buffer.push((digits.len() - 1 - index) / 2);
                //print!("{}", ((digits.len() - 1 - index) / 2));
            }
        } else {
            for _ in 0..value {
                //print!(".");
            }
        }
    }

    //print!("\nBuffer: ");
    //buffer.iter().for_each(|&value| print!("{}", value));
    //println!();

    // Destructure the struct into variables
    let Space { total, free } = disk_space;
    println!(
        "Disk space: Total: {}, Used: {}, Free: {}",
        total,
        total - free,
        free,
    );

    // 2333133121414131402
    // 022111222 (0,2,4,3,4,5,12,14,16) => 60
    'original: for (index, &value) in digits.iter().enumerate() {
        if global_index >= disk_space.total - disk_space.free {
            break 'original;
        }

        if index % 2 == 0 {
            for _ in 0..value {
                //print!("{}", (index / 2));
                sum += global_index * (index / 2);
                //print!("[{sum}]");

                global_index += 1;

                if global_index >= disk_space.total - disk_space.free {
                    break 'original;
                }
            }
        } else {
            for _ in 0..value {
                let copy_digit = buffer.remove(0);
                //print!("{copy_digit}");
                //print!(".");
                sum += global_index * copy_digit;
                //print!("[{sum}]");

                global_index += 1;

                if global_index >= disk_space.total - disk_space.free {
                    break 'original;
                }
            }
        }
    }

    println!();

    sum
}

fn get_part_2(digits: &[u8]) -> i32 {
    let mut sum = 0;

    let mut disk: Vec<char> = Vec::new();
    let mut files: Vec<(usize, usize, usize)> = Vec::new(); // index, index_len, digit
    let mut total_bytes = 0;

    for (i, &current_digit) in digits.iter().enumerate() {
        let repeat_count = current_digit as usize;
        let s = if i % 2 == 0 {
            (i / 2).to_string()
        } else {
            ".".to_string()
        };

        print!("{}", s.repeat(repeat_count)); // Repeating string

        for _ in 0..repeat_count {
            disk.extend(s.chars());
        }

        total_bytes += s.len() * repeat_count; // Adding to total bytes
    }

    let mut cummulative_index = 0;
    // 107,86,71,46,40,36,
    for i in 0..digits.len() {
        let digit = digits[i] as usize;
        let file_size = if i % 2 == 0 {
            digit * (i / 2).to_string().len()
        } else {
            digit
        };

        println!("file_size: {}", file_size);

        cummulative_index += file_size;
        println!("cummulative_index: {}", cummulative_index);

        if i % 2 == 0 {
            if file_size > 9 {
                continue;
            }

            files.push((cummulative_index, i / 2, digit));
        }
    }

    // for i in (0..digits.len()).step_by(2).rev() {
    //     let file_size = digits[i] as usize * (i / 2).to_string().len();

    //     if file_size > 9 {
    //         continue;
    //     }

    //     files.push((total_bytes - file_size, i / 2, digits[i]));
    // }

    println!();

    for file in &files {
        println!("{:?},", file);
    }

    println!();

    println!("\ntotal_bytes: {}", total_bytes);
    println!("{:?}", disk);

    sum
}
