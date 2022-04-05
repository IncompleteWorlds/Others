use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    if stdin.read_line(&mut buffer).is_err() {
        println!("ERROR: Reading line");
        return Ok(());
    }

    let numbers: Vec<i32> = buffer.split_whitespace()
                                .map(|v| v.parse::<i32>().unwrap_or(0) )
                                .collect();

    buffer.clear();
    if stdin.read_line(&mut buffer).is_err() {
        println!("ERROR: Reading line");
        return Ok(());
    }

    loop {
        // Process all number in the line
        let tmp_numbers: Vec<i32> = buffer.split_whitespace()
                                    .map(|v| v.parse::<i32>().unwrap_or(0) )
                                    .collect();

        for v in tmp_numbers {
            if numbers.contains(&v) {
                println!("True");
            } else {
                println!("False");
            }
        }

        buffer.clear();
        if stdin.read_line(&mut buffer).is_err() {
            println!("ERROR: Reading line");
            break;
        }
    }

    Ok(())
}
