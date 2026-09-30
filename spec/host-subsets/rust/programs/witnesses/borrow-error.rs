fn main() {
    let mut value = 1_i64;
    let first = &mut value;
    let second = &mut value;
    *first += 1;
    *second += 1;
    println!("{}", value);
}
