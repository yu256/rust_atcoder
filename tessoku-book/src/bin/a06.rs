#[proconio::fastout]
fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut sum: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + a[i];
    }

    for (left, right) in lr {
        println!("{}", sum[right] - sum[left - 1]);
    }
}
