fn sum_of_sqrs_of(nums: i64) -> i64 {
    (1..nums+1).fold(0, |sum, x| sum + x*x)
}

fn sqr_of_sums_of(nums: i64) -> i64 {
    let sum = (1..nums+1).fold(0, |sum, x| sum + x);
    sum * sum
}

fn main() {
    println!("{}",  sqr_of_sums_of(100) - sum_of_sqrs_of(100));
}
