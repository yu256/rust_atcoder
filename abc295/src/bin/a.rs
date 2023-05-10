use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: i32,
        w: [String; n],
    }

    let contains: bool = w.iter().any(|word: &String| ["and", "not", "that", "the", "you"].contains(&word.as_str()));

    print!("{}", if contains { "Yes" } else { "No" });
}