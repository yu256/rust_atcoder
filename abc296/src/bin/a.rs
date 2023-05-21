fn main() {
    proconio::input! {
        _n: usize,
        s: String,
    }

    let mut prev_char = '_';

    for c in s.chars() {
        if c == prev_char {
            print!("No");
            return;
        }
        prev_char = c;
    }
    print!("Yes");
}