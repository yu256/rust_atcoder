use proconio::{input, fastout};
 
#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        w: [usize; n],
    }

    let mut ans: usize = 0;

    for i in 1..n {
        if w[i] - w[i-1] <= d {
            ans = w[i];
            break
        }
    }

    let answer: isize = ans as isize;

    println!("{}", if answer != 0 { answer } else { -1 });
}
