// SPDX-License-Identifier: GPL-3.0-only
// Case: core/11-append-reverse. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// Append and reverse over sequence data: the section's list
/// operations translated to owned vectors.
fn append(first: Vec<i64>, second: Vec<i64>) -> Vec<i64> {
    let mut result = first;
    let mut index = 0;
    while index < second.len() {
        result.push(second[index]);
        index += 1;
    }
    result
}

fn reverse(items: Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    let mut index = items.len();
    while index > 0 {
        index -= 1;
        result.push(items[index]);
    }
    result
}

fn main() {
    let joined = append(vec![1, 2, 3], vec![4, 5]);
    println!("{}", joined.len());
    let flipped = reverse(joined);
    println!("{}", flipped[0]);
}
