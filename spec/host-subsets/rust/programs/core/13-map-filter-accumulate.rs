// SPDX-License-Identifier: GPL-3.0-only
// Case: core/13-map-filter-accumulate. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Higher-order iteration: map, filter, and accumulate over the
/// integer sequence, each taking an admitted operation as data.
fn map(items: &Vec<i64>, transform: fn(i64) -> i64) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut index = 0;
    while index < items.len() {
        result.push(transform(items[index]));
        index += 1;
    }
    result
}

fn filter(items: &Vec<i64>, keep: fn(i64) -> bool) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut index = 0;
    while index < items.len() {
        if keep(items[index]) {
            result.push(items[index]);
        }
        index += 1;
    }
    result
}

fn accumulate(items: &Vec<i64>, combine: fn(i64, i64) -> i64, start: i64) -> i64 {
    let mut total = start;
    let mut index = 0;
    while index < items.len() {
        total = combine(total, items[index]);
        index += 1;
    }
    total
}

fn square(x: i64) -> i64 {
    x * x
}

fn even(n: i64) -> bool {
    n % 2 == 0
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    let items = vec![1, 2, 3, 4, 5];
    let squares = map(&items, square);
    println!("{}", accumulate(&squares, add, 0));
    let evens = filter(&items, even);
    println!("{}", accumulate(&evens, add, 0));
}
