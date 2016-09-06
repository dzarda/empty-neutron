
fn fib_sum(to: i64) -> i64 {
	let mut prev = 1;
	let mut last = 2;
	let mut acc = 0;
	while last < to {
		if (last & 1) == 0 {
			acc += last;
		}
		let newlast = last + prev;
		prev = last;
		last = newlast;
	}
	acc
}

#[test]
fn test_fib_sum() {
	assert!(fib_sum(30) == 10);
}

fn main() {
	println!("{}", fib_sum(4_000_000));
}
