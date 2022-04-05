use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;

    let number_values = buffer.trim().parse::<i32>().unwrap_or(0);

    for _ in 0..number_values {
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        let list_words : Vec<&str> = buffer.trim().split_whitespace().collect();

        let mut reverse_line = String::new();

        for current_word in list_words {
            let mut reverse_word = String::new();

            for c in current_word.chars().rev() {
                reverse_word.push(c);
            }
            //current_word.clone_into(&mut reverse_word);

            reverse_line += reverse_word.as_str();
            reverse_line += " ";
        }

        println!("{}", reverse_line);
    }

    Ok(())
}
