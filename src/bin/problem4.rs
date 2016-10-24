fn is_palindrome(num: i32) -> bool {
	let str_num = num.to_string();
	let forward_iter = str_num.chars();
	let reverse_iter = str_num.chars().rev();
	forward_iter.eq(reverse_iter)
}


fn main() {
	let mut largest: i32 = 0;
	for num1 in (111..1000).rev() {
		for num2 in 111..1000 {
			let product = num1 * num2;
			if product > largest && is_palindrome(product) {
				largest = num1 * num2;
			}
		}
	}
	println!("{}", largest);
}
