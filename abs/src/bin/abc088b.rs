fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut alice = 0usize;
    let mut bob = 0usize;

    for (i, c) in a.iter().rev().enumerate() {
        if i % 2 == 0 {
            alice += c;
        } else {
            bob += c;
        }
    }

    println!("{}", alice - bob);
}
