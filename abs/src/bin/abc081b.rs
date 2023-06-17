fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut ans: Vec<usize> = vec![1; n];

    for i in &a {
        if i % 2 != 0 {
            println!("0");
            return;
        }
    }

    loop {
        for i in 0..n {
            a[i] = a[i] / 2;
            if a[i] % 2 == 0 {
                ans[i] += 1;
            } else {
                println!("{}", ans[i]);
                return;
            }
        }
    }
}
