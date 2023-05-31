fn main() {
    proconio::input! {
        s: [String; 8],
    }

    let mut column: usize = 0;
    let mut row: usize = 0;

    for i in 0..8 {
        for (index, c) in s[i].chars().enumerate() {
            if c == '*' {
                column = index;
                row = 8 - i;
            }
        }
    }
    println!("{}{}", get_char(column), row);
}

fn get_char(n: usize) -> char {
    let chars: Vec<char> = String::from("abcdefgh").chars().collect();
    chars[n]
}