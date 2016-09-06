fn sumto(to: i32) -> i64 {
	let mut acc = 0i64;
	for i in 1..to {
		if i % 3 == 0 || i % 5 == 0 {
			acc += i as i64;
		}
	}
	acc
}

#[test]
fn sumto_test() {
	assert!(sumto(10) == 23);
}

fn main() {
	println!("{}", sumto(1000));
}
