fn main() {
    proconio::input! {
        n: usize,
        y: usize,
    }

    let mut answer = None;

    'outer: for a in 0..=n {
        for b in 0..=n - a {
            let c = n - a - b;
            let total_amount = 10000 * a + 5000 * b + 1000 * c;

            if total_amount == y {
                answer = Some((a, b, c));
                break 'outer;
            }
        }
    }

    match answer {
        Some((a, b, c)) => println!("{} {} {}", a, b, c),
        _ => println!("-1 -1 -1")
    }
}