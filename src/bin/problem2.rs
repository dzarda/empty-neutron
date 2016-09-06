

fn fib_sum(to: i64) -> i64 {
	let mut prev = 1i64;
	let mut last = 2i64;
	let mut acc = 2i64;
	while last < to {
		if last % 2 == 0 {
			acc += last;
		}
		last = last + prev;
		prev = last;
	}
}

#[test]
fn test_fib_sum() {
	assert!(fib_sum(34) == 44);
}
