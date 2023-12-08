use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_FILE: &str = "assets/test-file.txt";

    #[test]
    fn test_read_file() {
        let fstring = read_file(EXAMPLE_FILE);
        assert_eq!(fstring, "secret number is 42");
    }

    #[test]
    fn test_read_lines() {
        if let Ok(lines) = read_lines(EXAMPLE_FILE) {
            for line in lines {
                if let Ok(_ip) = line {}
            }
        }
    }
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
