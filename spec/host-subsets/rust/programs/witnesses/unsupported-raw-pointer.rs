fn main() {
    let mut value = 3_i64;
    let raw = &mut value as *mut i64;
    unsafe { *raw += 1; }
    println!("{}", value);
}
