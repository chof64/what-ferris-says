use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let mut line = String::new();
    println!("What will Ferris say? ");
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let stdout = stdout();
    let width = line.chars().count();
    let mut writer = BufWriter::new(stdout.lock());

    say(&line, width, &mut writer).unwrap();
}
