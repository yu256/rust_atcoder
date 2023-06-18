fn main() {
    proconio::input! {
		_n: usize,
		s: proconio::marker::Chars,
	}

	let mut bool = false;

	for i in &s {
		if *i == 'o' {
			bool = true;
			break;
		}
	}

	for i in s {
		if i == 'x' {
			println!("No");
			return;
		}
	}

	println!("{}", if bool { "Yes" } else { "No" });
}
