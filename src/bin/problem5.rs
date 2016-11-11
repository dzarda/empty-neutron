fn is_evenly_divisible(num: i64) -> bool {
    for i in 2..21 {
        if (num % i) != 0 {
            return false;
        }
    }
    true
}

fn main() {
    for i in 1.. {
        if is_evenly_divisible(i) {
            println!("{}", i);
            return;
        }
    }
}
