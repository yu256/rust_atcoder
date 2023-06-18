fn main() {
    proconio::input! {
        n: u32,
        txy: [(i32, i32, i32); n],
    }

    println!("{}", if check(&txy) {"Yes"} else {"No"});

}

fn check(txy: &Vec<(i32, i32, i32)>) -> bool {
    let mut prev_t = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;

    for &(t, x, y) in txy {
        let dt = t - prev_t;
        let dx = (x - prev_x).abs();
        let dy = (y - prev_y).abs();

        if dx + dy > dt || (dt - dx - dy) % 2 != 0 {
            return false;
        }

        prev_t = t;
        prev_x = x;
        prev_y = y;
    }

    true
}