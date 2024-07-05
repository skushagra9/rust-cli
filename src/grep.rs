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
    find_match(content, &args.pattern, &mut std::io::stdout());
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    Ok(())
}

pub fn find_match<R: BufRead>(content: R, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        match line {
            Ok(line_content) => {
                if line_content.contains(pattern) {
                    writeln!(writer, "{}", line_content);
                }
            }
            Err(e) => {
                writeln!(writer, "Error reading line: {}", e);
            }
        }
    }
}
