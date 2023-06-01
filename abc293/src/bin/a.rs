fn main() {
    proconio::input! {
        s: String,
    }

    let mut ans = String::new();
    let mut i = 0;
    loop {
        ans.push(s.chars().nth(i + 1).unwrap());
        ans.push(s.chars().nth(i).unwrap());
        i += 2;
        if i >= s.len() {
            print!("{}", ans);
            break;
        }
    }
}