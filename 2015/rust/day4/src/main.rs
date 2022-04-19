use md5;
use std::str;

const INPUT: &str = "yzbqklnj";

fn main() {
    let mut i = 0;
    let mut solution1: String = "".to_owned();
    let mut solution2: String = "".to_owned();
    loop {
        if i > 100000000 {
            break;
        }
        let digest = md5::compute(format!("{}{}", INPUT, i));
        let s_digest = format!("{:x}", digest);
        if s_digest.starts_with("00000") {
            if solution1 == "" {
                solution1 = s_digest.clone();
            }
            if s_digest.starts_with("000000") {
                solution2 = s_digest.clone();
                break;
            }
        }
        i += 1;
    }
    println!("solution1: {}, solution2: {}", solution1, solution2);
}
