// SPDX-License-Identifier: GPL-3.0-only
// Case: core/18-strings-display. Provenance: derived from teaching execution;
// parent confirms natively. Never native-proven.

/// The section's string discipline: owned strings, borrowed slices,
/// and ordered display effects.
fn describe(name: &str, count: i64) -> String {
    let mut text = String::from(name);
    text.push_str(" x ");
    if count == 1 {
        text.push_str("one");
    } else {
        text.push_str("many");
    }
    text
}

fn main() {
    let one = describe("widget", 1);
    println!("{}", one);
    let many = describe("gadget", 7);
    println!("{}", many.as_str());
    println!("{}", many.len() > 0);
}
