use std::io::Write;
#[test]
fn test_find_matches() {
    let content = "This is the first line.\nPattern match here.\nThis is the third line.";
    let pattern = "Pattern";

    let mut buffer = Vec::new();
    find_match(content, pattern, &mut buffer);

    let result_str = String::from_utf8_lossy(&buffer);
    assert!(result_str.contains("Pattern match here."));
    assert!(!result_str.contains("This is the first line."));
    assert!(!result_str.contains("This is the third line."));
}

pub fn find_match(content: &str, pattern: &str, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("Failed to write line");
        }
    }
}
