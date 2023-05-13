use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: isize,
        k: isize,
    }
    
    let mut count: usize = 0;
    for i in 1..=n {
        for j in 1..=n {
            let m: isize = k - i - j;
            if 1 <= m && m <= n {
                count += 1;
            }
        }
    }
    print!("{}", count);
}