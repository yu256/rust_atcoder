fn main() {
    proconio::input! {
        s: String,
    }

    println!("{}", if check(&s) {"YES"} else {"NO"});

}

fn check(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    for word in vec!["dream", "dreamer", "erase", "eraser"] {
        if s.starts_with(word) {
            let rest = &s[word.len()..];
            if check(rest) {
                return true;
            }
        }
    }

    false
}