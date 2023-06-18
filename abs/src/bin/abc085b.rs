fn main() {
    proconio::input! {
        n: usize,
        mut d: [usize; n],
    }

    d.sort();

    let mut prev = 0usize;
    let mut ans = 0usize;

    for  n in d.iter().rev() {
        if prev != *n {
            ans += 1;
            prev = *n;
        }
    }

    println!("{}", ans);
}
