fn main() {
    proconio::input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    
    print!("{}", if a.contains(&x) {"Yes"} else {"No"})
}