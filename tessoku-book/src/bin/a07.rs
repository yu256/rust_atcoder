#[proconio::fastout]
fn main() {
    proconio::input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut attendees = vec![0; d + 1];

    for i in 0..n {
        let (l, r) = lr[i];
        for j in l..=r {
            attendees[j] += 1;
        }
    }

    for i in 1..=d {
        println!("{}", attendees[i]);
    }
}