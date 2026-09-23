// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.9, listing: running a listing from the workspace it lives in.

fn main() {
    let name = ch00::sec_0_9::crate_name();
    println!("{name}");
    // => ch00
    assert_eq!(name, "ch00");
}
