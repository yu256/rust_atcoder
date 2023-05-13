use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
        mut b:[Chars; r],
    }

    let is_destroyed = |(x1, y1): (usize, usize), (x2, y2): (usize, usize), d: i64| -> bool {
        (x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs() <= d
    };
    
    for i in 0..r {
        for j in 0..c {
            if let Ok(d) = b[i][j].to_string().parse::<i64>() {
                if d != 0 {
                    b[i][j] = '.';
                    for x in 0..r {
                        for y in 0..c {
                            if b[x][y] == '#' && is_destroyed((i, j), (x, y), d) {
                                b[x][y] = '.';
                            }
                        }
                    }
                }
            }
        }
    }
    
    for i in 0..r {
        for j in 0..c {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
