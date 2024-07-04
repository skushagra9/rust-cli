use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
#[derive(Parser)]

pub struct GrepArgs {
    pattern: String,
    path: std::path::PathBuf,
}

pub fn process_file(args: &GrepArgs) -> io::Result<()> {
    let file = File::open(&args.path).expect("could not open file");
    let content = BufReader::new(file);
    for line in content.lines() {
        match line {
            Ok(line_content) => {
                if line_content.contains(&args.pattern) {
                    println!("{}", line_content);
                }
            }
            Err(e) => {
                println!("Error reading line: {}", e);
            }
        }
    }
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    Ok(())
}
