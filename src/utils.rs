use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_lines(filename: String) -> Vec<String> {
    let mut file_lines: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| match line {
            Ok(l) => file_lines.push(l),
            Err(_) => todo!(),
        })
    }
    file_lines
}
