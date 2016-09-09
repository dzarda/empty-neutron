
// Stolen from http://stackoverflow.com/a/412942
fn prime_factors(mut n: i64) -> Vec<i64> {
	let mut factors = Vec::new();
	let mut d = 2;
	while n > 1 {
		while n % d == 0 {
			factors.push(d);
			n /= d;
		}
		d += 1;
	}
	factors
}


fn main() {
	println!("{}", match prime_factors(600851475143).pop() 
		{ Some(n) => n, None => -1 }
	);
}
