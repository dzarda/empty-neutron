//use std;

fn fizz_buzz(i: i32) -> &'static str {
	match i {
		i if i % 15 == 0 => "FizzBuzz",
		i if i % 3 == 0 => "Fizz",
		i if i % 5 == 0 => "Buzz",
		_ => "nada"
	}
}

fn main() {
	for i in 0..10 {
		println!("{}", fizz_buzz(i));
	}
}
