fn squared_sum(array: &[i32]) -> i32 {
    let mut sum = 0;
    for num in array {
        sum += num * num;
    }
    return sum;
}

fn main() {
    println!("Hello, world!");
    println!("{}", squared_sum(&[1, 2, 3]))
}
