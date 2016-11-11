fn is_evenly_divisible(num: i64) -> bool {
    for i in 2..21 {
        if (num % i) != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut i = 2;
    while !is_evenly_divisible(i) {
        i += 2;
    }
    println!("{:?}", i);
}
