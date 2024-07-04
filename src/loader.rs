use std::thread::sleep;
use std::time::Duration;
pub fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        pb.inc(1);
        sleep(Duration::from_millis(50));
    }
    pb.finish_with_message("done");
}
