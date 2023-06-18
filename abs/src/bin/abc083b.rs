fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut sums = Vec::new();

    for i in 1..=n {
        let mut sum = 0;
        let mut num = i;

        while num > 0 {
            sum += num % 10;
            num /= 10;
        }

        if a <= sum && sum <= b {
            sums.push(i);
        }
    }

    let ans: usize = sums.iter().sum();
    println!("{}", ans);
}
